import React, { useState, useEffect } from 'react';
import { Github, RefreshCw, Clock, Settings2, CheckCircle2, AlertCircle, Database, Server, Code2, Cpu, GitMerge, ChevronDown, BarChart2, Search, FileText, Trash2, Wifi, WifiOff } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { GitGraphVisualization } from './GitGraphVisualization';
import { CommitHeatmap } from './CommitHeatmap';
import { RepositoryActivityChart } from './RepositoryActivityChart';
import { RepositoryLineChart } from './RepositoryLineChart';
import { SyncStatusDonutChart } from './SyncStatusDonutChart';
import { ConflictResolutionModal } from './ConflictResolutionModal';

const getRelativeTimestamp = (dateString: string) => {
  const diff = Date.now() - new Date(dateString).getTime();
  if (diff < 0) return 'Just now';
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (seconds < 60) return `${seconds}s ago`;
  if (minutes < 60) return `${minutes}m ago`;
  if (hours < 24) return `${hours}h ago`;
  return `${days}d ago`;
};

interface RepoSyncConfig {
  id: string;
  name: string;
  owner: string;
  description: string;
  icon: React.ElementType;
  autoSync: boolean;
  frequency: '15m' | '1h' | '6h' | '24h';
  lastSync: string | null;
  lastCommit?: string;
  stars?: number;
  status: 'idle' | 'syncing' | 'success' | 'error';
  branches?: string[];
  activeBranch?: string;
  latestCommitMessage?: string;
  latestCommitAuthor?: string;
  isEcosystem?: boolean;
  subRepoIds?: string[];
}

const INITIAL_REPOS: RepoSyncConfig[] = [
  {
    id: 'atown-os',
    name: 'a-townchain-os',
    owner: 'atown-chain',
    description: 'Core operating system kernel and foundational architecture.',
    icon: Cpu,
    autoSync: true,
    frequency: '1h',
    lastSync: new Date(Date.now() - 1000 * 60 * 30).toISOString(),
    lastCommit: new Date(Date.now() - 1000 * 60 * 60 * 24 * 5).toISOString(),
    stars: 1024,
    status: 'success',
    branches: ['main', 'develop', 'feature/networking'],
    activeBranch: 'main',
    latestCommitMessage: 'Optimize kernel scheduler',
    latestCommitAuthor: 'Alice'
  },
  {
    id: 'atown-vm',
    name: 'atvm-sandbox',
    owner: 'atown-chain',
    description: 'A-Town Virtual Machine and smart contract execution environment.',
    icon: Server,
    autoSync: true,
    frequency: '15m',
    lastSync: new Date(Date.now() - 1000 * 60 * 5).toISOString(),
    lastCommit: new Date(Date.now() - 1000 * 60 * 60 * 24 * 45).toISOString(),
    stars: 256,
    status: 'success',
    branches: ['main', 'release/1.0'],
    activeBranch: 'main',
    latestCommitMessage: 'Fix sandbox escape vulnerability',
    latestCommitAuthor: 'Bob'
  },
  {
    id: 'atown-contracts',
    name: 'atc-smart-contracts',
    owner: 'atown-community',
    description: 'Standard library and governance smart contracts.',
    icon: Code2,
    autoSync: false,
    frequency: '24h',
    lastSync: new Date(Date.now() - 1000 * 60 * 60 * 48).toISOString(),
    lastCommit: new Date(Date.now() - 1000 * 60 * 60 * 24 * 12).toISOString(),
    stars: 512,
    status: 'error',
    branches: ['main', 'feat/governance'],
    activeBranch: 'main',
    latestCommitMessage: 'Update consensus parameter bounds',
    latestCommitAuthor: 'Charlie'
  },
  {
    id: 'atown-frontend',
    name: 'atown-explorer-ui',
    owner: 'atown-chain',
    description: 'Block explorer and network dashboard frontend.',
    icon: Database,
    autoSync: false,
    frequency: '6h',
    lastSync: null,
    lastCommit: new Date(Date.now() - 1000 * 60 * 60 * 24 * 60).toISOString(),
    stars: 128,
    status: 'idle',
    branches: ['main', 'next-ui', 'bugfix/sidebar'],
    activeBranch: 'main',
    latestCommitMessage: 'Add dark mode toggle',
    latestCommitAuthor: 'Dave'
  }
];

const translations = {
  EN: {
    title: 'GitHub Repository Sync',
    description: 'Configure automated synchronization settings for individual A-TownChain repositories. Manage sync frequencies and monitor connection status per module.',
    searchPlaceholder: 'Search repositories...',
    orgPlaceholder: 'Filter by organization/username...',
    syncLogs: 'Sync Logs',
    sortBy: 'Sort by:',
    name: 'Name',
    stars: 'Stars',
    lastCommitDate: 'Last Commit Date',
    activeRepository: 'Active Repository',
    selectBranch: 'Select Branch',
    repositoryActivity: 'Repository Activity',
    showActivityHeatmap: 'Show Activity Heatmap',
    showOnlyActive: 'Show only active',
    hideInactive: 'Hide Inactive',
    syncAllRepositories: 'Sync All Repositories',
    repositories: 'Repositories',
    visualizeGitGraph: 'Visualize Git Graph',
    systemSyncHistory: 'System Sync History',
    timestamp: 'Timestamp',
    repository: 'Repository',
    status: 'Status',
    duration: 'Duration',
    details: 'Details',
    reason: 'Reason',
    syncSelected: 'Sync Selected',
    exportCsv: 'Export CSV',
    latestCommit: 'Latest Commit',
    autoSync: 'Auto-Sync',
    frequency: 'Frequency',
    every15m: 'Every 15 mins',
    every1h: 'Every 1 hour',
    every6h: 'Every 6 hours',
    daily: 'Daily',
    syncing: 'Syncing...',
    synced: 'Synced',
    failed: 'Failed',
    idle: 'Idle',
    lastSynced: 'Last synced:',
    neverSynced: 'Never synced',
    lastCommit: 'Last commit:',
    noCommits: 'No commits',
    syncNow: 'Sync Now',
    successText: 'Success',
    allStatus: 'All Status',
    retrySelected: 'Retry Selected',
    deleteLogs: 'Delete Logs',
    loadOrgRepos: 'Load Repos',
    orgLoadPlaceholder: 'GitHub Organization or User...',
    groupSelected: 'Group Selected',
    groupToEcosystem: 'Merge into Ecosystem',
    ecosystemName: 'Ecosystem Name',
    cancel: 'Cancel',
    createCcosystem: 'Create Ecosystem'
  },
  DE: {
    title: 'GitHub Repository Sync',
    description: 'Konfiguriere automatische Synchronisierungseinstellungen für einzelne A-TownChain Repositories. Verwalte Synchronisierungsfrequenzen und überwache den Verbindungsstatus pro Modul.',
    searchPlaceholder: 'Repositories suchen...',
    orgPlaceholder: 'Nach Organisation/Benutzernamen filtern...',
    syncLogs: 'Sync-Protokolle',
    sortBy: 'Sortieren nach:',
    name: 'Name',
    stars: 'Sterne',
    lastCommitDate: 'Letzter Commit',
    activeRepository: 'Aktives Repository',
    selectBranch: 'Branch auswählen',
    repositoryActivity: 'Repository-Aktivität',
    showActivityHeatmap: 'Aktivitäts-Heatmap anzeigen',
    showOnlyActive: 'Nur aktive anzeigen',
    hideInactive: 'Inaktive ausblenden',
    syncAllRepositories: 'Alle Repositories synchronisieren',
    repositories: 'Repositories',
    visualizeGitGraph: 'Git-Graph visualisieren',
    systemSyncHistory: 'System-Synchronisierungsverlauf',
    timestamp: 'Zeitstempel',
    repository: 'Repository',
    status: 'Status',
    duration: 'Dauer',
    details: 'Details',
    reason: 'Grund',
    syncSelected: 'Ausgewählte synchronisieren',
    exportCsv: 'CSV exportieren',
    latestCommit: 'Letzter Commit',
    autoSync: 'Auto-Sync',
    frequency: 'Frequenz',
    every15m: 'Alle 15 Min',
    every1h: 'Jede Stunde',
    every6h: 'Alle 6 Stunden',
    daily: 'Täglich',
    syncing: 'Synchronisiere...',
    synced: 'Synchronisiert',
    failed: 'Fehlgeschlagen',
    idle: 'Leerlauf',
    lastSynced: 'Zuletzt synchronisiert:',
    neverSynced: 'Nie synchronisiert',
    lastCommit: 'Letzter Commit:',
    noCommits: 'Keine Commits',
    syncNow: 'Jetzt synchronisieren',
    successText: 'Erfolgreich',
    allStatus: 'Alle Status',
    retrySelected: 'Auswahl wiederholen',
    deleteLogs: 'Protokolle löschen',
    loadOrgRepos: 'Laden',
    orgLoadPlaceholder: 'GitHub Org oder User...',
    groupSelected: 'Zusammenführen',
    groupToEcosystem: 'In Ökosystem zusammenführen',
    ecosystemName: 'Name des Ökosystems',
    cancel: 'Abbrechen',
    createCcosystem: 'Ökosystem erstellen'
  }
};

export function GitHubRepoSyncView({ language = 'DE' }: { language?: 'DE' | 'EN' }) {
  const t = translations[language];

  const [repos, setRepos] = useState<RepoSyncConfig[]>(INITIAL_REPOS);
  const [hideInactive, setHideInactive] = useState(false);
  const [showOnlyActive, setShowOnlyActive] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [orgQuery, setOrgQuery] = useState('');
  const [isFetchingOrg, setIsFetchingOrg] = useState(false);
  const [showEcosystemModal, setShowEcosystemModal] = useState(false);
  const [ecosystemName, setEcosystemName] = useState('');

  const fetchOrgRepos = async () => {
    if (!orgQuery || isFetchingOrg) return;
    setIsFetchingOrg(true);
    
    try {
      let res = await fetch(`https://api.github.com/orgs/${orgQuery}/repos?per_page=100`);
      if (res.status === 404) {
         res = await fetch(`https://api.github.com/users/${orgQuery}/repos?per_page=100`);
      }
      
      if (res.ok) {
        const data = await res.json();
        const newRepos: RepoSyncConfig[] = data.map((repo: any) => ({
          id: repo.id.toString(),
          name: repo.name,
          owner: repo.owner?.login || orgQuery,
          description: repo.description?.substring(0, 100) || 'No description provided.',
          icon: Code2,
          autoSync: false,
          frequency: '24h',
          lastSync: null,
          lastCommit: repo.pushed_at || repo.updated_at,
          stars: repo.stargazers_count,
          status: 'idle',
          branches: [repo.default_branch || 'main'],
          activeBranch: repo.default_branch || 'main',
        }));
        
        setRepos(prev => {
          const map = new Map(prev.map(p => [p.id, p]));
          newRepos.forEach(r => map.set(r.id, r));
          return Array.from(map.values());
        });
      }
    } catch (e) {
      console.error('Failed to fetch org repos', e);
    } finally {
      setIsFetchingOrg(false);
    }
  };

  const createEcosystem = () => {
    if (ecosystemName && selectedRepos.length > 0) {
      const mergedId = 'eco-' + Date.now();
      const groupedRepos = repos.filter(r => selectedRepos.includes(r.id));
      const newEcosystem: RepoSyncConfig = {
        id: mergedId,
        name: ecosystemName,
        owner: 'ecosystem',
        description: `Ecosystem containing ${selectedRepos.length} bundled repositories.`,
        icon: Database,
        autoSync: true,
        frequency: '24h',
        lastSync: null,
        lastCommit: new Date().toISOString(),
        stars: groupedRepos.reduce((acc, r) => acc + (r.stars || 0), 0),
        status: 'success',
        branches: ['main'],
        activeBranch: 'main',
        isEcosystem: true,
        subRepoIds: selectedRepos
      };
      
      setRepos(prev => {
        const next = prev.filter(r => !selectedRepos.includes(r.id));
        return [newEcosystem, ...next];
      });
      setSelectedRepos([]);
      setShowEcosystemModal(false);
      setEcosystemName('');
    }
  };

  const [selectedRepos, setSelectedRepos] = useState<string[]>([]);
  const [activeTab, setActiveTab] = useState<'Repositories'|'Visualize Git Graph'|'Repository Activity'|'Sync Logs'>('Repositories');
  const [sortOrder, setSortOrder] = useState<'Name' | 'Stars' | 'LastCommitDate'>('Name');
  const [expandedRepoId, setExpandedRepoId] = useState<string | null>(null);
  const [activeRepoId, setActiveRepoId] = useState<string>(INITIAL_REPOS[0].id);

  const activeRepo = repos.find(r => r.id === activeRepoId) || repos[0];

  const [syncLogs, setSyncLogs] = useState([
    { id: 'log-1', time: new Date().toISOString(), repo: 'atown-os', status: 'success', duration: '1,240ms', details: 'Fetched 3 new commits.', reason: '' },
    { id: 'log-2', time: new Date(Date.now() - 1000 * 60 * 5).toISOString(), repo: 'atvm-sandbox', status: 'success', duration: '840ms', details: 'Already up-to-date.', reason: '' },
    { id: 'log-3', time: new Date(Date.now() - 1000 * 60 * 60 * 2).toISOString(), repo: 'atc-smart-contracts', status: 'error', duration: '5,021ms', details: 'Connection timeout, retrying.', reason: 'Error: Connection reset by peer' },
    { id: 'log-4', time: new Date(Date.now() - 1000 * 60 * 60 * 4).toISOString(), repo: 'atown-explorer-ui', status: 'success', duration: '2,100ms', details: 'Fetched 12 new commits.', reason: '' },
  ]);

  const [selectedLogs, setSelectedLogs] = useState<string[]>([]);
  const [logStatusFilter, setLogStatusFilter] = useState<'all' | 'success' | 'error'>('all');

  const [githubSyncStep, setGithubSyncStep] = useState(0);
  const [notionalSyncStep, setNotionalSyncStep] = useState(0);
  const [isSyncingAll, setIsSyncingAll] = useState(false);
  const [estimatedTimeRemaining, setEstimatedTimeRemaining] = useState<number | null>(null);
  const [showRawLogsOverlay, setShowRawLogsOverlay] = useState(false);
  const [logFilterRepo, setLogFilterRepo] = useState<string>('all');
  const [logFilterOutcome, setLogFilterOutcome] = useState<string>('all');
  const [logSearchQuery, setLogSearchQuery] = useState('');
  const [isOnline, setIsOnline] = useState(typeof window !== 'undefined' ? window.navigator.onLine : true);
  const logsContainerRef = React.useRef<HTMLDivElement>(null);
  const [showConflictModal, setShowConflictModal] = useState(false);

  useEffect(() => {
    if (showRawLogsOverlay && logsContainerRef.current) {
        logsContainerRef.current.scrollTop = logsContainerRef.current.scrollHeight;
    }
  }, [syncLogs, showRawLogsOverlay]);

  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  React.useEffect(() => {
    let interval: any;
    const isAnySyncing = repos.some(r => r.status === 'syncing');
    if (isAnySyncing) {
      interval = setInterval(() => {
        setGithubSyncStep(v => Math.min(100, v + 2 + Math.random() * 5));
        setNotionalSyncStep(v => Math.min(100, v + 1 + Math.random() * 8));
      }, 100);
    } else {
      if (githubSyncStep > 0 || notionalSyncStep > 0) {
        setGithubSyncStep(100);
        setNotionalSyncStep(100);
        setTimeout(() => {
          setGithubSyncStep(0);
          setNotionalSyncStep(0);
        }, 1000);
      }
    }
    return () => clearInterval(interval);
  }, [repos, githubSyncStep, notionalSyncStep]);

  const toggleAutoSync = (id: string) => {
    setRepos(prev => prev.map(repo => 
      repo.id === id ? { ...repo, autoSync: !repo.autoSync } : repo
    ));
  };

  const updateFrequency = (id: string, frequency: RepoSyncConfig['frequency']) => {
    setRepos(prev => prev.map(repo => 
      repo.id === id ? { ...repo, frequency } : repo
    ));
  };

  const toggleRepoSelection = (id: string) => {
    setSelectedRepos(prev => 
      prev.includes(id) ? prev.filter(rId => rId !== id) : [...prev, id]
    );
  };

  const selectAllRepos = (displayedIds: string[]) => {
    if (selectedRepos.length === displayedIds.length) {
      setSelectedRepos([]);
    } else {
      setSelectedRepos(displayedIds);
    }
  };

  const filteredLogs = syncLogs.filter(log => logStatusFilter === 'all' || log.status === logStatusFilter);

  const exportCsv = () => {
    const headers = [t.timestamp, t.repository, t.status, t.duration, t.details, t.reason];
    const rows = filteredLogs.map(log => [
      new Date(log.time).toLocaleString(),
      log.repo,
      log.status,
      log.duration,
      log.details,
      log.reason || ''
    ]);
    const csvContent = "data:text/csv;charset=utf-8," 
      + [headers.join(","), ...rows.map(e => e.map(item => `"${item}"`).join(","))].join("\n");
      
    const encodedUri = encodeURI(csvContent);
    const link = document.createElement("a");
    link.setAttribute("href", encodedUri);
    link.setAttribute("download", "sync_logs.csv");
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const toggleLogSelection = (id: string) => {
    setSelectedLogs(prev => 
      prev.includes(id) ? prev.filter(lId => lId !== id) : [...prev, id]
    );
  };

  const selectAllLogs = () => {
    if (selectedLogs.length === filteredLogs.length) {
      setSelectedLogs([]);
    } else {
      setSelectedLogs(filteredLogs.map(l => l.id));
    }
  };

  const deleteSelectedLogs = () => {
    setSyncLogs(prev => prev.filter(log => !selectedLogs.includes(log.id)));
    setSelectedLogs([]);
  };

  const retrySelectedLogs = () => {
    // In a real app this would trigger the sync again. For the UI, we could update the status.
    setSyncLogs(prev => prev.map(log => {
      if (selectedLogs.includes(log.id)) {
        return { ...log, status: 'syncing', duration: '...', details: 'Retrying sync...' };
      }
      return log;
    }));
    
    // Simulate completion
    setTimeout(() => {
      setSyncLogs(prev => prev.map(log => {
        if (selectedLogs.includes(log.id)) {
          return { ...log, status: 'success', duration: Math.floor(Math.random() * 2000 + 500) + 'ms', details: 'Sync successful after retry.', reason: '' };
        }
        return log;
      }));
      setSelectedLogs([]);
    }, 1500);
  };

  const syncRepo = (id: string, attempt = 1): Promise<void> => {
    return new Promise((resolve) => {
      setRepos(prev => prev.map(repo => 
        repo.id === id ? { ...repo, status: 'syncing' } : repo
      ));

      setTimeout(() => {
        const shouldFail = Math.random() < 0.3 && attempt <= 3;

        if (shouldFail) {
          setRepos(prev => prev.map(repo => 
            repo.id === id ? { ...repo, status: 'error' } : repo
          ));

          const delay = Math.pow(2, attempt) * 1000;
          
          setSyncLogs(prev => [{
              id: `retry-${Date.now()}-${Math.random()}`,
              time: new Date().toISOString(),
              repo: repos.find(r => r.id === id)?.name || id,
              status: 'error',
              duration: 'failed',
              details: `Sync failed (attempt ${attempt}). Retrying in ${delay / 1000}s...`,
              reason: 'Network Timeout'
          }, ...prev]);

          setTimeout(() => {
            syncRepo(id, attempt + 1).then(resolve);
          }, delay);
        } else {
          setRepos(prev => prev.map(repo => 
            repo.id === id ? { ...repo, status: 'success', lastSync: new Date().toISOString() } : repo
          ));
          
          if (attempt > 1) {
             setSyncLogs(prev => [{
                  id: `recovered-${Date.now()}-${Math.random()}`,
                  time: new Date().toISOString(),
                  repo: repos.find(r => r.id === id)?.name || id,
                  status: 'success',
                  duration: `${Math.floor(2000 + Math.random() * 2000)}ms`,
                  details: `Recovered sync successfully on attempt ${attempt}.`,
                  reason: ''
              }, ...prev]);
          }
          resolve();
        }
      }, 2000 + Math.random() * 2000);
    });
  };

  const displayedRepos = repos.filter(repo => {
    if (searchQuery && !repo.name.toLowerCase().includes(searchQuery.toLowerCase())) return false;
    if (orgQuery && !repo.owner.toLowerCase().includes(orgQuery.toLowerCase())) return false;
    if (hideInactive && repo.lastSync === null) return false;
    if (showOnlyActive) {
      if (!repo.lastCommit) return false;
      const thirtyDaysAgo = Date.now() - 30 * 24 * 60 * 60 * 1000;
      if (new Date(repo.lastCommit).getTime() < thirtyDaysAgo) {
        return false;
      }
    }
    return true;
  }).sort((a, b) => {
    if (sortOrder === 'Name') {
      return a.name.localeCompare(b.name);
    }
    if (sortOrder === 'Stars') {
      return (b.stars || 0) - (a.stars || 0);
    }
    if (sortOrder === 'LastCommitDate') {
      const timeA = a.lastCommit ? new Date(a.lastCommit).getTime() : 0;
      const timeB = b.lastCommit ? new Date(b.lastCommit).getTime() : 0;
      return timeB - timeA;
    }
    return 0;
  });

  const startSequentialSync = async () => {
    if (isSyncingAll) return;
    const pendingRepos = displayedRepos.filter(r => r.status !== 'success');
    if (pendingRepos.length === 0) return;
    
    setIsSyncingAll(true);
    let remaining = pendingRepos.length * 2; // roughly 2s per repo
    setEstimatedTimeRemaining(remaining);
    
    const countInterval = setInterval(() => {
        setEstimatedTimeRemaining(prev => Math.max(0, (prev || 0) - 1));
    }, 1000);

    for (const repo of pendingRepos) {
        await syncRepo(repo.id);
    }
    
    clearInterval(countInterval);
    setIsSyncingAll(false);
    setEstimatedTimeRemaining(null);
  };

  const syncSelectedOperations = () => {
    selectedRepos.forEach(repoId => syncRepo(repoId));
  };

  return (
    <div className="flex flex-col h-full bg-[#04060b] p-8 overflow-y-auto custom-scrollbar">
      <div className="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4 mb-8">
        <div>
          <h1 className="text-2xl font-mono text-white flex items-center gap-3 tracking-tight">
            <Github className="w-7 h-7 text-atc-cyan" />
            {t.title}
          </h1>
          <p className="text-slate-400 mt-2 text-sm max-w-2xl">
            {t.description}
          </p>
        </div>
        
        <div className="sidebar-menu flex flex-row items-center gap-4 flex-wrap">
          <div className="relative flex-1 min-w-[200px]">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input 
              type="text" 
              placeholder={t.searchPlaceholder}
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-black/30 border border-white/10 text-slate-300 text-sm font-medium py-1.5 pl-9 pr-3 rounded-lg focus:outline-none focus:border-atc-cyan"
            />
          </div>

          <div className="flex items-center gap-2 flex-1 min-w-[200px]">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
              <input 
                type="text" 
                placeholder={t.orgLoadPlaceholder}
                value={orgQuery}
                onKeyDown={(e) => { if (e.key === 'Enter') fetchOrgRepos(); }}
                onChange={(e) => setOrgQuery(e.target.value)}
                className="w-full bg-black/30 border border-white/10 text-slate-300 text-sm font-medium py-1.5 pl-9 pr-3 rounded-lg focus:outline-none focus:border-atc-cyan"
              />
            </div>
            <button
              onClick={fetchOrgRepos}
              disabled={!orgQuery || isFetchingOrg}
              className="px-4 py-1.5 bg-atc-cyan/10 hover:bg-atc-cyan/20 border border-atc-cyan/30 text-atc-cyan text-xs font-mono font-bold rounded-lg transition-colors disabled:opacity-50 flex items-center gap-2"
            >
              <Database className="w-4 h-4" />
              {t.loadOrgRepos}
            </button>
          </div>

          <button
            onClick={() => setActiveTab('Sync Logs')}
            className={`flex items-center gap-2 px-4 py-2 text-xs font-mono font-bold tracking-wider uppercase rounded-lg transition-colors ${
              activeTab === 'Sync Logs'
                ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30'
                : 'bg-white/5 hover:bg-white/10 border border-white/10 text-white'
            }`}
          >
            <FileText className="w-4 h-4" />
            {t.syncLogs}
          </button>

          <div className="flex items-center gap-2">
            <div className="flex items-center gap-1.5 px-3 py-1.5 bg-black/40 border border-white/10 rounded-lg text-xs font-mono shadow-inner">
              {isOnline ? (
                <div className="flex items-center gap-1.5 text-emerald-400">
                  <Wifi className="w-3.5 h-3.5" />
                  <span className="hidden sm:inline uppercase tracking-wider font-bold">Online</span>
                </div>
              ) : (
                <div className="flex items-center gap-1.5 text-red-400">
                  <WifiOff className="w-3.5 h-3.5" />
                  <span className="hidden sm:inline uppercase tracking-wider font-bold">Offline</span>
                </div>
              )}
              <div className="w-px h-4 bg-white/10 mx-1" />
              {isSyncingAll || repos.some(r => r.status === 'syncing') ? (
                <div className="flex items-center gap-1.5 text-indigo-400">
                  <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                  <span className="uppercase tracking-wider font-bold">Syncing</span>
                </div>
              ) : repos.some(r => r.status === 'error') ? (
                <div className="flex items-center gap-1.5 text-amber-400">
                  <AlertCircle className="w-3.5 h-3.5" />
                  <span className="uppercase tracking-wider font-bold">Error</span>
                </div>
              ) : (
                <div className="flex items-center gap-1.5 text-slate-400">
                  <Clock className="w-3.5 h-3.5" />
                  <span className="uppercase tracking-wider font-bold">Idle</span>
                </div>
              )}
            </div>
            <button
              onClick={() => setShowRawLogsOverlay(true)}
              className="flex items-center gap-2 px-4 py-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 text-xs font-mono font-bold tracking-wider uppercase rounded-lg transition-colors"
            >
              <Code2 className="w-4 h-4" />
              View Sync Logs
            </button>
          </div>

          <select
            value={sortOrder}
            onChange={(e) => setSortOrder(e.target.value as 'Name' | 'Stars' | 'LastCommitDate')}
            className="bg-black/30 border border-white/10 text-slate-300 text-sm font-medium py-1.5 px-3 rounded-lg focus:outline-none focus:border-atc-cyan"
          >
            <option value="Name">{t.sortBy} {t.name}</option>
            <option value="Stars">{t.sortBy} {t.stars}</option>
            <option value="LastCommitDate">{t.sortBy} {t.lastCommitDate}</option>
          </select>

          <select
            value={activeRepoId}
            onChange={(e) => setActiveRepoId(e.target.value)}
            className="bg-black/30 border border-white/10 text-slate-300 text-sm font-medium py-1.5 px-3 rounded-lg focus:outline-none focus:border-atc-cyan"
            title={t.activeRepository}
          >
            {repos.map(r => <option key={r.id} value={r.id}>{r.name}</option>)}
          </select>

          {activeRepo.branches && (
            <select
              value={activeRepo.activeBranch || ''}
              onChange={(e) => {
                setRepos(prev => prev.map(repo => 
                  repo.id === activeRepo.id ? { ...repo, activeBranch: e.target.value } : repo
                ));
              }}
              className="bg-black/30 border border-white/10 text-slate-300 text-sm font-medium py-1.5 px-3 rounded-lg focus:outline-none focus:border-pink-500"
              title="Active Branch"
            >
              <option value="" disabled>{t.selectBranch}</option>
              {activeRepo.branches.map(branch => (
                <option key={branch} value={branch}>{branch}</option>
              ))}
            </select>
          )}

          <button
            onClick={() => setActiveTab('Repository Activity')}
            className={`flex items-center gap-2 px-4 py-2 text-xs font-mono font-bold tracking-wider uppercase rounded-lg transition-colors ${
              activeTab === 'Repository Activity'
                ? 'bg-teal-500/20 text-teal-400 border border-teal-500/30'
                : 'bg-white/5 hover:bg-white/10 border border-white/10 text-white'
            }`}
          >
            <BarChart2 className="w-4 h-4" />
            {t.repositoryActivity}
          </button>

          <button
            onClick={() => {
              setActiveTab('Repositories');
              setExpandedRepoId(activeRepoId === expandedRepoId ? null : activeRepoId);
            }}
            className="flex items-center gap-2 px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 text-white text-xs font-mono font-bold tracking-wider uppercase rounded-lg transition-colors"
          >
            {t.showActivityHeatmap}
          </button>

          <label className="flex items-center gap-2 text-sm text-slate-300 font-medium cursor-pointer">
            <input 
              type="checkbox" 
              checked={showOnlyActive} 
              onChange={(e) => setShowOnlyActive(e.target.checked)}
              className="accent-atc-cyan cursor-pointer"
            />
            {t.showOnlyActive}
          </label>

          <label className="flex items-center gap-2 text-sm text-slate-300 font-medium cursor-pointer">
            <input 
              type="checkbox" 
              checked={hideInactive} 
              onChange={(e) => setHideInactive(e.target.checked)}
              className="accent-atc-cyan cursor-pointer"
            />
            {t.hideInactive}
          </label>
          {selectedRepos.length > 0 && (
            <div className="flex items-center gap-2">
              <button 
                onClick={syncSelectedOperations}
                className="flex items-center gap-2 px-5 py-2.5 bg-atc-cyan/10 text-atc-cyan font-mono text-sm tracking-wider uppercase border border-atc-cyan/30 rounded-xl hover:bg-atc-cyan/20 transition-colors shadow-[0_0_15px_rgba(6,182,212,0.1)]"
              >
                <RefreshCw className="w-4 h-4" />
                {t.syncSelected} ({selectedRepos.length})
              </button>
              <button 
                onClick={() => setShowEcosystemModal(true)}
                className="flex items-center gap-2 px-5 py-2.5 bg-emerald-500/10 text-emerald-400 font-mono text-sm tracking-wider uppercase border border-emerald-500/30 rounded-xl hover:bg-emerald-500/20 transition-colors shadow-[0_0_15px_rgba(16,185,129,0.1)]"
              >
                <Database className="w-4 h-4" />
                {t.groupSelected} ({selectedRepos.length})
              </button>
            </div>
          )}
          <button 
            onClick={startSequentialSync}
            disabled={isSyncingAll}
            className="flex items-center gap-2 px-5 py-2.5 bg-indigo-500/10 text-indigo-400 font-mono text-sm tracking-wider uppercase border border-indigo-500/30 rounded-xl hover:bg-indigo-500/20 transition-colors shadow-[0_0_15px_rgba(99,102,241,0.1)] disabled:opacity-50"
          >
            <RefreshCw className={`w-4 h-4 ${isSyncingAll ? 'animate-spin' : ''}`} />
            {isSyncingAll && estimatedTimeRemaining !== null 
              ? `Syncing... (~${estimatedTimeRemaining}s)` 
              : t.syncAllRepositories}
          </button>
          <button 
            onClick={() => setShowConflictModal(true)}
            className="flex items-center gap-2 px-5 py-2.5 bg-amber-500/10 text-amber-500 font-mono text-sm tracking-wider uppercase border border-amber-500/30 rounded-xl hover:bg-amber-500/20 transition-colors shadow-[0_0_15px_rgba(245,158,11,0.1)]"
          >
            <AlertCircle className="w-4 h-4" />
            Resolve State Conflicts
          </button>
        </div>
      </div>

      <div className="flex mb-8 border-b border-white/10">
        <button
          onClick={() => setActiveTab('Repositories')}
          className={`px-4 py-2 font-mono text-sm font-bold uppercase tracking-wider transition-colors border-b-2 ${
            activeTab === 'Repositories'
              ? 'border-atc-cyan text-atc-cyan'
              : 'border-transparent text-slate-500 hover:text-slate-300'
          }`}
        >
          {t.repositories}
        </button>
        <button
          onClick={() => setActiveTab('Visualize Git Graph')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm font-bold uppercase tracking-wider transition-colors border-b-2 ${
            activeTab === 'Visualize Git Graph'
              ? 'border-pink-500 text-pink-400'
              : 'border-transparent text-slate-500 hover:text-slate-300'
          }`}
        >
          <GitMerge className="w-4 h-4" />
          {t.visualizeGitGraph}
        </button>
      </div>

      {activeTab === 'Repository Activity' && (
        <div className="flex-1 min-h-[400px] grid grid-cols-1 lg:grid-cols-2 gap-6 items-start content-start">
          <RepositoryActivityChart repoId={activeRepoId} language={language} />
          <RepositoryLineChart repoId={activeRepoId} language={language} />
        </div>
      )}

      {activeTab === 'Visualize Git Graph' && (
        <div className="flex-1 min-h-[400px]">
          <GitGraphVisualization />
        </div>
      )}

      {activeTab === 'Sync Logs' && (
        <div className="flex-1 bg-[#090b14]/80 backdrop-blur-md rounded-2xl border border-white/5 p-6 flex flex-col gap-6">
          <div className="flex items-start justify-between">
            <h3 className="text-xl font-mono text-white flex items-center gap-3 tracking-tight">
              <FileText className="w-6 h-6 text-indigo-400" />
              {t.systemSyncHistory}
            </h3>
            <div className="flex flex-wrap items-center gap-4">
              <SyncStatusDonutChart language={language} />

              <div className="h-8 w-px bg-white/10 mx-2"></div>

              <select
                value={logStatusFilter}
                onChange={(e) => setLogStatusFilter(e.target.value as any)}
                className="bg-black/30 border border-white/10 text-slate-300 text-xs font-mono py-2 px-3 rounded-lg focus:outline-none focus:border-atc-cyan"
              >
                <option value="all">{t.allStatus}</option>
                <option value="success">{t.successText}</option>
                <option value="error">{t.failed}</option>
              </select>

              {selectedLogs.length > 0 && (
                <div className="flex items-center gap-2">
                  <button 
                    onClick={retrySelectedLogs}
                    className="flex items-center gap-2 px-3 py-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 font-mono text-xs font-bold uppercase tracking-wider rounded-lg transition-colors"
                  >
                    <RefreshCw className="w-3.5 h-3.5" /> {t.retrySelected}
                  </button>
                  <button 
                    onClick={deleteSelectedLogs}
                    className="flex items-center gap-2 px-3 py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 border border-red-500/30 font-mono text-xs font-bold uppercase tracking-wider rounded-lg transition-colors"
                  >
                    <Trash2 className="w-3.5 h-3.5" /> {t.deleteLogs}
                  </button>
                </div>
              )}

              <button 
                onClick={exportCsv}
                className="flex items-center gap-2 px-4 py-2 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 font-mono text-xs font-bold uppercase tracking-wider rounded-lg transition-colors ml-auto"
              >
                <FileText className="w-4 h-4" />
                {t.exportCsv}
              </button>
            </div>
          </div>
          
          <div className="flex-1 overflow-y-auto custom-scrollbar border border-white/5 rounded-xl">
            <table className="w-full text-left border-collapse">
              <thead className="bg-[#00101c]/80 sticky top-0 z-10 backdrop-blur-md">
                <tr className="border-b border-white/10 text-xs font-mono font-bold tracking-wider text-slate-500 uppercase">
                  <th className="py-3 pl-4 w-12">
                    <input 
                      type="checkbox" 
                      className="accent-atc-cyan cursor-pointer"
                      checked={selectedLogs.length === filteredLogs.length && filteredLogs.length > 0}
                      onChange={selectAllLogs}
                    />
                  </th>
                  <th className="py-3">{t.timestamp}</th>
                  <th className="py-3">{t.repository}</th>
                  <th className="py-3">{t.status}</th>
                  <th className="py-3">{t.duration}</th>
                  <th className="py-3">{t.details}</th>
                  <th className="py-3 pr-4">{t.reason}</th>
                </tr>
              </thead>
              <tbody className="text-sm">
                {filteredLogs.map((log) => (
                  <tr key={log.id} className="border-b border-white/5 hover:bg-white/[0.02] transition-colors">
                    <td className="py-4 pl-4 w-12">
                      <input 
                        type="checkbox" 
                        className="accent-atc-cyan cursor-pointer"
                        checked={selectedLogs.includes(log.id)}
                        onChange={() => toggleLogSelection(log.id)}
                      />
                    </td>
                    <td className="py-4 text-slate-400 font-mono text-xs">{new Date(log.time).toLocaleTimeString(window.navigator.language)}</td>
                    <td className="py-4 font-mono text-atc-cyan">{log.repo}</td>
                    <td className="py-4">
                      {log.status === 'success' ? (
                        <span className="flex items-center gap-1.5 text-emerald-400 text-xs"><CheckCircle2 className="w-3.5 h-3.5" /> {t.successText}</span>
                      ) : log.status === 'syncing' ? (
                        <span className="flex items-center gap-1.5 text-indigo-400 text-xs"><RefreshCw className="w-3.5 h-3.5 animate-spin" /> {t.syncing || 'Syncing...'}</span>
                      ) : (
                        <span className="flex items-center gap-1.5 text-red-400 text-xs"><AlertCircle className="w-3.5 h-3.5" /> {t.failed}</span>
                      )}
                    </td>
                    <td className="py-4 text-slate-400 font-mono text-xs">{log.duration}</td>
                    <td className="py-4 text-slate-400">{log.details}</td>
                    <td className="py-4 pr-4">
                       {log.reason && (
                         <div className="relative group flex items-center justify-start cursor-help">
                           <AlertCircle className="w-4 h-4 text-red-400 opacity-80 group-hover:opacity-100 transition-opacity" />
                           <div className="absolute left-6 top-1/2 -translate-y-1/2 w-48 bg-red-500/10 border border-red-500/20 text-red-200 text-xs p-2 rounded opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity z-10">
                             {log.reason}
                           </div>
                         </div>
                       )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {activeTab === 'Repositories' && (
        <>
        <style>{`
        @keyframes slide-right {
          0% { transform: translateX(-100%); }
          100% { transform: translateX(300%); }
        }
      `}</style>
      
      {/* Visual Sync Progress Bar & Timeline */}
      <div className="mb-8 p-6 bg-[#090b14]/80 backdrop-blur-md rounded-2xl border border-white/5 space-y-6">
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
          <div className="flex flex-col p-4 bg-black/40 rounded-xl border border-white/5">
            <span className="text-xs text-slate-500 font-mono uppercase tracking-wider mb-1">Total Tracked</span>
            <span className="text-2xl text-white font-mono font-bold">{repos.length}</span>
          </div>
          <div className="flex flex-col p-4 bg-emerald-500/5 rounded-xl border border-emerald-500/10">
            <span className="text-xs text-emerald-500/70 font-mono uppercase tracking-wider mb-1">Successfully Synced</span>
            <span className="text-2xl text-emerald-400 font-mono font-bold">{repos.filter(r => r.status === 'success').length}</span>
          </div>
          <div className="flex flex-col p-4 bg-red-500/5 rounded-xl border border-red-500/10">
            <span className="text-xs text-red-500/70 font-mono uppercase tracking-wider mb-1">Failed Attempts</span>
            <span className="text-2xl text-red-400 font-mono font-bold">{syncLogs.filter(l => l.status === 'error').length}</span>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex justify-between items-center text-xs font-mono tracking-widest text-slate-400">
            <span>GITHUB SYNC PROGRESS</span>
            <span className="text-atc-cyan">{Math.floor(githubSyncStep)}%</span>
          </div>
          <div className="h-1.5 bg-black rounded-full overflow-visible relative">
            <motion.div 
              className={`h-full bg-atc-cyan rounded-full ${repos.some(r => r.status === 'syncing') ? 'shadow-[0_0_15px_rgba(34,211,238,0.8)]' : ''}`}
              initial={{ width: 0 }}
              animate={{ width: `${githubSyncStep}%` }}
              transition={{ duration: 0.5, ease: "easeOut" }}
            />
          </div>

          <div className="flex justify-between items-center text-xs font-mono tracking-widest text-slate-400">
            <span>NOTIONAL SYNC PROGRESS</span>
            <span className="text-pink-500">{Math.floor(notionalSyncStep)}%</span>
          </div>
          <div className="h-1.5 bg-black rounded-full overflow-visible relative">
            <motion.div 
              className={`h-full bg-pink-500 rounded-full ${repos.some(r => r.status === 'syncing') ? 'shadow-[0_0_15px_rgba(236,72,153,0.8)]' : ''}`}
              initial={{ width: 0 }}
              animate={{ width: `${notionalSyncStep}%` }}
              transition={{ duration: 0.5, ease: "easeOut" }}
            />
          </div>
        </div>

        <div>
          <div className="text-xs font-mono tracking-widest text-slate-400 mb-4 uppercase">Recent Sync Operations</div>
          <div className="flex flex-col gap-3">
            {syncLogs.slice(0, 8).map(log => (
              <div key={log.id} className="flex items-center justify-between p-4 rounded-xl border border-white/5 bg-black/20 hover:bg-black/30 transition-colors">
                <div className="flex items-center gap-4">
                  <div className={`w-2 h-2 rounded-full shadow-lg ${log.status === 'success' ? 'bg-emerald-400 shadow-emerald-400/50' : log.status === 'error' ? 'bg-red-400 shadow-red-400/50' : 'bg-indigo-400 shadow-indigo-400/50 animate-pulse'}`} />
                  <div>
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-bold text-white font-mono">{log.repo}</span>
                      <span className={`text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded-full ${log.status === 'success' ? 'bg-emerald-500/10 text-emerald-400' : log.status === 'error' ? 'bg-red-500/10 text-red-400' : 'bg-indigo-500/10 text-indigo-400'}`}>
                        {log.status}
                      </span>
                    </div>
                    <div className="text-xs text-slate-400 mt-1 max-w-md truncate" title={log.details}>{log.details}</div>
                  </div>
                </div>
                <div className="flex flex-col items-end gap-1 text-right">
                  <div className="text-xs font-mono text-slate-300">{new Date(log.time).toLocaleString(window.navigator.language)}</div>
                  <div className="text-[10px] font-mono text-slate-500">Duration: {log.duration}</div>
                  {log.reason && <div className="text-[10px] text-red-400 max-w-[200px] truncate" title={log.reason}>{log.reason}</div>}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="grid grid-cols-[repeat(auto-fit,minmax(min(100%,450px),1fr))] gap-6">
          {displayedRepos.map((repo) => (
            <motion.div 
              key={repo.id}
              layout
              onMouseEnter={() => setActiveRepoId(repo.id)}
              className={`p-6 rounded-2xl border bg-[#090b14]/80 backdrop-blur-md relative overflow-hidden transition-all duration-300 group ${
                repo.status === 'syncing' 
                  ? 'border-indigo-500/50 shadow-[0_0_20px_rgba(99,102,241,0.15)]' 
                  : 'border-white/5 hover:border-white/10'
              }`}
            >
              {/* Hover Preview Card */}
              {repo.latestCommitMessage && (
                <div className="absolute top-4 right-14 max-w-[240px] bg-[#00101c]/95 border border-atc-cyan/30 p-3 rounded-xl shadow-2xl z-50 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none backdrop-blur-xl">
                  <div className="text-[9px] text-atc-cyan font-mono tracking-widest uppercase mb-1">{t.latestCommit}</div>
                  <div className="text-sm text-slate-200 mb-2 truncate max-w-[200px]" title={repo.latestCommitMessage}>{repo.latestCommitMessage}</div>
                  <div className="flex items-center gap-2">
                    <div className="w-5 h-5 rounded-full bg-slate-800 flex items-center justify-center text-[10px] text-white border border-white/10 scale-110">{(repo.latestCommitAuthor?.[0] || 'U').toUpperCase()}</div>
                    <span className="text-xs text-slate-400 font-mono ml-1">{repo.latestCommitAuthor}</span>
                  </div>
                </div>
              )}

            {/* Visual Sync Status Indicator */}
            <div className="absolute top-0 inset-x-0 h-1.5 bg-black/50 overflow-hidden">
              {repo.status === 'syncing' && (
                <div className="h-full w-full bg-indigo-500/20">
                  <div className="h-full w-1/3 bg-indigo-500 shadow-[0_0_10px_rgba(99,102,241,0.8)] rounded-full" style={{ animation: 'slide-right 1.5s infinite ease-in-out' }} />
                </div>
              )}
              {repo.status === 'success' && (
                <div className="h-full w-full bg-emerald-500/80 shadow-[0_0_10px_rgba(16,185,129,0.8)]" />
              )}
              {repo.status === 'error' && (
                <div className="h-full w-full bg-red-500/80 shadow-[0_0_10px_rgba(239,68,68,0.8)]" />
              )}
              {repo.status === 'idle' && (
                <div className="h-full w-full bg-slate-600/30" />
              )}
            </div>

            <div className="flex justify-between items-start mt-2 mb-6">
              <div className="flex items-start gap-4 flex-1 min-w-0 pr-4">
                <input 
                  type="checkbox" 
                  checked={selectedRepos.includes(repo.id)} 
                  onChange={() => toggleRepoSelection(repo.id)}
                  className="mt-4 accent-atc-cyan cursor-pointer shrink-0"
                />
                <div className="p-3 rounded-xl bg-slate-800/80 border border-slate-700/50 text-slate-300 shadow-inner shrink-0">
                  <repo.icon className="w-6 h-6" />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-3 mb-1">
                    <h3 className="text-lg font-bold text-white font-mono truncate">{repo.name}</h3>
                    {repo.isEcosystem && (
                      <span className="px-2 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 text-[10px] font-bold tracking-wider font-mono uppercase flex items-center gap-1.5 shrink-0">
                        <Database className="w-3 h-3" />
                         Ecosystem
                      </span>
                    )}
                    <div className="flex items-center gap-1.5 px-2 py-1 rounded-full bg-slate-800/80 border border-slate-700 shrink-0">
                      <div className={`w-2 h-2 rounded-full ${repo.status === 'success' ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]' : repo.status === 'syncing' ? 'bg-indigo-500 shadow-[0_0_8px_rgba(99,102,241,0.5)] animate-pulse' : repo.status === 'error' ? 'bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.5)]' : 'bg-slate-500'}`} />
                      <span className={`text-[10px] uppercase font-bold tracking-wider leading-none ${repo.status === 'success' ? 'text-emerald-400' : repo.status === 'syncing' ? 'text-indigo-400' : repo.status === 'error' ? 'text-red-400' : 'text-slate-400'}`}>
                        {repo.status === 'success' ? 'Success' : repo.status === 'syncing' ? 'Pending' : repo.status === 'error' ? 'Failed' : 'Idle'}
                      </span>
                    </div>
                  </div>
                  <div className="text-xs text-atc-cyan/80 font-mono mb-1 truncate">{repo.owner}</div>
                  <p className="text-sm text-slate-400 mt-1 max-w-md line-clamp-2">{repo.description}</p>
                </div>
              </div>
              <button 
                onClick={() => setExpandedRepoId(expandedRepoId === repo.id ? null : repo.id)}
                className="p-2 rounded-lg hover:bg-white/5 text-slate-400 transition-colors shrink-0"
              >
                <ChevronDown className={`w-5 h-5 transition-transform duration-300 ${expandedRepoId === repo.id ? 'rotate-180' : ''}`} />
              </button>
            </div>

            <div className="flex flex-col sm:flex-row justify-between gap-6 pt-6 border-t border-white/5 w-full">
              
              {/* Sync Configuration */}
              <div className="space-y-4 w-full sm:w-auto">
                <div className="flex items-center justify-between gap-4">
                  <div className="flex items-center gap-2 text-sm text-slate-300 font-medium whitespace-nowrap">
                    <Settings2 className="w-4 h-4 text-slate-500" />
                    {t.autoSync}
                  </div>
                  <button 
                    onClick={() => toggleAutoSync(repo.id)}
                    className={`relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors focus:outline-none ${repo.autoSync ? 'bg-atc-cyan' : 'bg-slate-700'}`}
                  >
                    <span className={`inline-block h-3.5 w-3.5 transform rounded-full bg-white transition-transform ${repo.autoSync ? 'translate-x-4.5' : 'translate-x-1'}`} />
                  </button>
                </div>

                <div className="flex items-center justify-between gap-4 opacity-100 transition-opacity" style={{ opacity: repo.autoSync ? 1 : 0.5, pointerEvents: repo.autoSync ? 'auto' : 'none' }}>
                  <div className="flex items-center gap-2 text-sm text-slate-300 font-medium whitespace-nowrap">
                    <Clock className="w-4 h-4 text-slate-500" />
                    {t.frequency}
                  </div>
                  <select 
                    value={repo.frequency}
                    onChange={(e) => updateFrequency(repo.id, e.target.value as RepoSyncConfig['frequency'])}
                    className="bg-black/30 border border-white/10 text-slate-300 text-xs font-mono py-1.5 px-3 rounded-lg focus:outline-none focus:border-atc-cyan max-w-[140px]"
                  >
                    <option value="15m">{t.every15m}</option>
                    <option value="1h">{t.every1h}</option>
                    <option value="6h">{t.every6h}</option>
                    <option value="24h">{t.daily}</option>
                  </select>
                </div>
              </div>

              {/* Sync Status */}
              <div className="flex flex-col justify-between items-start sm:items-end w-full sm:w-auto">
                <div className="flex flex-col items-start sm:items-end gap-2 text-left sm:text-right w-full">
                  <div className="text-xs text-slate-500 font-mono uppercase tracking-wider">{t.status}</div>
                  
                  {repo.status === 'syncing' ? (
                    <div className="flex flex-col gap-2 w-full sm:w-[140px]">
                      <div className="flex items-center gap-2 text-indigo-400 font-mono text-xs px-2 py-1 bg-indigo-500/10 border border-indigo-500/20 rounded-md w-fit sm:ml-auto">
                        <RefreshCw className="w-3 h-3 animate-spin shrink-0" />
                        {t.syncing}
                      </div>
                      <div className="w-full h-1 bg-black rounded-full overflow-hidden relative">
                        <div className="absolute inset-y-0 left-0 bg-indigo-500 rounded-full shadow-[0_0_8px_rgba(99,102,241,0.8)]" style={{ width: '60%', animation: 'slide-right 2s infinite ease-in-out' }} />
                      </div>
                      <div className="text-[9px] font-mono text-indigo-400/80 uppercase tracking-wider text-left sm:text-right">
                        Cloning chunks...
                      </div>
                    </div>
                  ) : repo.status === 'success' ? (
                    <div className="flex flex-col items-start sm:items-end">
                      <div className="flex items-center gap-2 text-emerald-400 font-mono text-sm px-3 py-1 bg-emerald-500/10 border border-emerald-500/20 rounded-md">
                        <CheckCircle2 className="w-3.5 h-3.5 shrink-0" />
                        {t.synced}
                      </div>
                      {repo.lastSync && (
                        <div className="text-[10px] text-atc-cyan mt-1.5 font-mono">
                          {getRelativeTimestamp(repo.lastSync)}
                        </div>
                      )}
                    </div>
                  ) : repo.status === 'error' ? (
                    <div className="flex flex-col items-start sm:items-end gap-1.5">
                      <div className="flex items-center gap-2 text-red-400 font-mono text-sm px-3 py-1 bg-red-500/10 border border-red-500/20 rounded-md">
                        <AlertCircle className="w-3.5 h-3.5 shrink-0" />
                        {t.failed}
                      </div>
                      <button 
                         onClick={() => syncRepo(repo.id)} 
                         className="flex items-center justify-center gap-1.5 px-3 py-1 bg-red-500/10 hover:bg-red-500/20 text-red-400 text-xs font-bold rounded-md border border-red-500/30 transition-colors shrink-0 whitespace-nowrap"
                      >
                         <RefreshCw className="w-3 h-3" /> Retry Sync
                      </button>
                    </div>
                  ) : (
                    <div className="flex flex-col items-start sm:items-end">
                      <div className="flex items-center gap-2 text-slate-400 font-mono text-sm px-3 py-1 bg-white/5 border border-white/10 rounded-md">
                        <Clock className="w-3.5 h-3.5 shrink-0" />
                        {t.idle}
                      </div>
                      {repo.lastSync && (
                        <div className="text-[10px] text-slate-500 mt-1.5 font-mono">
                          {getRelativeTimestamp(repo.lastSync)}
                        </div>
                      )}
                    </div>
                  )}

                  <div className="text-[10px] text-slate-500 mt-2">
                    {!repo.lastSync && <>{t.neverSynced}<br /></>}
                    {repo.lastCommit ? `${t.lastCommit} ${new Date(repo.lastCommit).toLocaleString(window.navigator.language)}` : t.noCommits}
                  </div>
                </div>

                <div className="flex justify-end mt-4 w-full">
                  <button 
                    onClick={() => syncRepo(repo.id)}
                    disabled={repo.status === 'syncing'}
                    className="flex items-center justify-center gap-2 px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 text-white text-xs font-mono font-bold tracking-wider uppercase rounded-lg transition-colors disabled:opacity-50 w-full sm:w-auto shrink-0 whitespace-nowrap"
                  >
                    <RefreshCw className={`w-3.5 h-3.5 ${repo.status === 'syncing' ? 'animate-spin' : ''}`} />
                    {t.syncNow}
                  </button>
                </div>
              </div>

            </div>
            
            <AnimatePresence>
              {expandedRepoId === repo.id && (
                <motion.div
                  initial={{ height: 0, opacity: 0 }}
                  animate={{ height: 'auto', opacity: 1 }}
                  exit={{ height: 0, opacity: 0 }}
                  className="overflow-hidden"
                >
                  <CommitHeatmap repoId={repo.id} />
                </motion.div>
              )}
            </AnimatePresence>

          </motion.div>
        ))}
      </div>
      </>
      )}

      <AnimatePresence>
        {showRawLogsOverlay && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-50 flex items-center justify-center p-8 bg-black/60 backdrop-blur-sm"
            onClick={() => setShowRawLogsOverlay(false)}
          >
            <motion.div
              initial={{ scale: 0.95 }}
              animate={{ scale: 1 }}
              exit={{ scale: 0.95 }}
              onClick={(e) => e.stopPropagation()}
              className="bg-[#0b0e17] border border-white/10 rounded-2xl w-full max-w-4xl max-h-full flex flex-col shadow-2xl"
            >
              <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between p-6 border-b border-white/10 gap-4">
                <div className="flex flex-col sm:flex-row items-start sm:items-center gap-4 w-full">
                  <h2 className="text-xl font-mono text-white flex items-center gap-2 whitespace-nowrap">
                    <Code2 className="w-5 h-5 text-indigo-400" />
                    Raw Sync Logs JSON
                  </h2>
                  <div className="flex flex-wrap items-center gap-2 flex-1">
                    <div className="relative flex-1 min-w-[150px] max-w-sm">
                      <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-slate-500" />
                      <input
                        type="text"
                        placeholder="Search logs..."
                        value={logSearchQuery}
                        onChange={(e) => setLogSearchQuery(e.target.value)}
                        className="w-full bg-black/50 border border-white/10 text-white text-xs font-mono rounded-lg pl-9 pr-3 py-2 outline-none focus:border-atc-cyan/50 placeholder:text-slate-600"
                      />
                    </div>
                    <select
                      value={logFilterOutcome}
                      onChange={(e) => setLogFilterOutcome(e.target.value)}
                      className="bg-black/50 border border-white/10 text-white text-xs font-mono rounded-lg px-3 py-2 outline-none focus:border-atc-cyan/50"
                    >
                      <option value="all">All Outcomes</option>
                      <option value="success">Success Only</option>
                      <option value="error">Error Only</option>
                    </select>
                    <select
                      value={logFilterRepo}
                      onChange={(e) => setLogFilterRepo(e.target.value)}
                      className="bg-black/50 border border-white/10 text-white text-xs font-mono rounded-lg px-3 py-2 outline-none focus:border-atc-cyan/50 max-w-[200px] truncate"
                    >
                      <option value="all">All Repositories</option>
                      {Array.from(new Set(syncLogs.map(l => l.repo))).map(repo => (
                        <option key={repo} value={repo}>{repo}</option>
                      ))}
                    </select>
                  </div>
                </div>
                <button
                  onClick={() => setShowRawLogsOverlay(false)}
                  className="px-4 py-2 bg-white/5 hover:bg-white/10 text-slate-400 rounded-lg transition-colors text-xs font-bold uppercase tracking-wider shrink-0"
                >
                  Close
                </button>
              </div>
              <div ref={logsContainerRef} className="flex-1 overflow-auto p-6 custom-scrollbar bg-black/50 rounded-b-2xl">
                <pre className="text-xs font-mono text-green-400 whitespace-pre-wrap break-all">
                  {JSON.stringify(
                    syncLogs.filter(log => {
                      if (logFilterRepo !== 'all' && log.repo !== logFilterRepo) return false;
                      if (logFilterOutcome !== 'all' && log.status !== logFilterOutcome) return false;
                      if (logSearchQuery) {
                        const q = logSearchQuery.toLowerCase();
                        if (!log.details.toLowerCase().includes(q) && 
                            !log.reason.toLowerCase().includes(q) &&
                            !log.repo.toLowerCase().includes(q)) {
                          return false;
                        }
                      }
                      return true;
                    }),
                    null, 
                    2
                  )}
                </pre>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      <AnimatePresence>
        {showEcosystemModal && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-[100] bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 xl:p-8"
          >
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 20 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 20 }}
              className="bg-[#0b0f19] border border-white/10 rounded-2xl w-full max-w-md shadow-2xl flex flex-col"
            >
              <div className="p-6 border-b border-white/10 flex items-center gap-3">
                <Database className="w-5 h-5 text-emerald-400" />
                <h3 className="text-xl font-mono text-white tracking-tight">{t.groupToEcosystem}</h3>
              </div>
              <div className="p-6 flex flex-col gap-4">
                <p className="text-sm font-mono text-slate-400">
                  You are about to merge <span className="text-white font-bold">{selectedRepos.length}</span> repositories into a single ecosystem group.
                </p>
                <div className="relative">
                  <input
                    type="text"
                    value={ecosystemName}
                    onChange={(e) => setEcosystemName(e.target.value)}
                    placeholder={t.ecosystemName}
                    className="w-full bg-black/50 border border-white/10 rounded-xl px-4 py-3 text-white font-mono focus:outline-none focus:border-emerald-500/50"
                  />
                </div>
              </div>
              <div className="p-6 border-t border-white/10 flex items-center justify-between gap-4">
                <button
                  onClick={() => setShowEcosystemModal(false)}
                  className="px-6 py-2.5 bg-white/5 hover:bg-white/10 text-white rounded-xl text-sm font-mono tracking-wider uppercase transition-colors"
                >
                  {t.cancel}
                </button>
                <button
                  onClick={createEcosystem}
                  disabled={!ecosystemName.trim()}
                  className="px-6 py-2.5 bg-emerald-500/20 hover:bg-emerald-500/30 border border-emerald-500/30 text-emerald-400 rounded-xl text-sm font-mono tracking-wider uppercase transition-colors disabled:opacity-50"
                >
                  {t.createCcosystem}
                </button>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      <ConflictResolutionModal 
        isOpen={showConflictModal} 
        onClose={() => setShowConflictModal(false)} 
        onResolve={(resolutions) => {
          console.log('Resolutions applied:', resolutions);
          setShowConflictModal(false);
          // Logic to apply resolutions
        }}
      />
    </div>
  );
}
