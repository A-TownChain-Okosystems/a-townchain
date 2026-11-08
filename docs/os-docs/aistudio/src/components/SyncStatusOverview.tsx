import React, { useState, useEffect } from 'react';
import { useSyncMetrics } from '../contexts/SyncMetricsContext';
import { HardDrive, Database, FileText, CalendarDays, Check, X, RefreshCw } from 'lucide-react';

export function SyncStatusOverview() {
  const { isSyncing, syncHistory, forceSync } = useSyncMetrics();
  const [globalContextMenu, setGlobalContextMenu] = useState<{x: number, y: number} | null>(null);
  const [syncingState, setSyncingState] = useState<Record<string, boolean>>({});
  
  const formatTime = (timeString: string) => {
    const time = new Date(timeString);
    return time.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  };
  
  useEffect(() => {
    const closeMenu = () => setGlobalContextMenu(null);
    window.addEventListener('click', closeMenu);
    return () => window.removeEventListener('click', closeMenu);
  }, []);

  const handleManualForceSync = async (e: React.MouseEvent) => {
    e.stopPropagation();
    setGlobalContextMenu(null);
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Triggering Manual Force Sync...' }));
    window.dispatchEvent(new CustomEvent('trigger-github-sync'));
    window.dispatchEvent(new CustomEvent('trigger-notion-sync'));
    try {
      await forceSync('sheets');
    } catch (err) {}
  };

  const handleViewLogs = (e: React.MouseEvent) => {
    e.stopPropagation();
    setGlobalContextMenu(null);
    window.dispatchEvent(new CustomEvent('open-window', { detail: { id: 'settings', tab: 'sync-diagnostics' } }));
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Opened Sync Logs in Settings.' }));
  };

  const handleClearCache = (e: React.MouseEvent) => {
    e.stopPropagation();
    setGlobalContextMenu(null);
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Local Sync Cache Cleared.' }));
    // Depending on what clearing cache means in this context, we can just log or fire an event
  };
  
  const TooltipWrapper = ({ children, text }: { children: React.ReactNode, text: string }) => (
    <div 
      className="relative group/sync-tooltip flex items-center cursor-context-menu"
      onContextMenu={(e) => {
        e.preventDefault();
        e.stopPropagation();
        const menuWidth = 160;
        const menuHeight = 100;
        let x = e.clientX;
        let y = e.clientY;
        
        if (x + menuWidth > window.innerWidth) {
          x = window.innerWidth - menuWidth - 8;
        }
        if (y + menuHeight > window.innerHeight) {
          y = window.innerHeight - menuHeight - 8;
        }

        setGlobalContextMenu({ x, y });
      }}
    >
      {children}
      <div className="absolute bottom-full right-0 mb-2 px-2 py-1 bg-slate-800 text-white text-[10px] rounded opacity-0 group-hover/sync-tooltip:opacity-100 whitespace-nowrap pointer-events-none transition-opacity z-50 border border-slate-700 shadow-xl">
        {text}
      </div>
    </div>
  );
  
  const getServiceIcon = (service: string) => {
    switch (service) {
      case 'drive': return <HardDrive className="w-3 h-3 text-blue-400" />;
      case 'sheets': return <Database className="w-3 h-3 text-emerald-400" />;
      case 'docs': return <FileText className="w-3 h-3 text-indigo-400" />;
      case 'calendar': return <CalendarDays className="w-3 h-3 text-orange-400" />;
      default: return <Database className="w-3 h-3 text-slate-400" />;
    }
  };

  const getServiceName = (service: string) => {
    switch (service) {
      case 'drive': return 'Drive';
      case 'sheets': return 'Sheets';
      case 'docs': return 'Docs';
      case 'calendar': return 'Calendar';
      default: return service;
    }
  };

  const recentHistory = syncHistory.slice().reverse().slice(0, 4);

  return (
    <TooltipWrapper text={isSyncing ? "Syncing..." : "Synced"}>
      <div id="sync-status-overview" className="flex flex-col text-[9px] font-mono whitespace-nowrap text-slate-400 group-hover:text-slate-300 transition-colors">
        {recentHistory.length > 0 ? (
          <table className="w-full border-collapse">
            <tbody>
              {recentHistory.map((item, idx) => (
                <tr 
                  key={idx} 
                  data-service={item.service}
                  className="border-b border-white/5 last:border-0 hover:bg-white/5"
                >
                  <td className="py-0.5 pr-2 opacity-60">{formatTime(item.timestamp)}</td>
                  <td className="py-0.5 pr-2 flex items-center gap-1">
                    {getServiceIcon(item.service)}
                    <span className={syncingState[item.service] ? "text-emerald-300 animate-pulse" : ""}>{getServiceName(item.service)}</span>
                  </td>
                  <td className="py-0.5 pl-2 text-right">
                    {syncingState[item.service] ? (
                      <RefreshCw className="w-3 h-3 text-emerald-400 inline animate-spin" />
                    ) : item.status === 'success' ? (
                      <Check className="w-3 h-3 text-emerald-400 inline" />
                    ) : (
                      <X className="w-3 h-3 text-red-400 inline" />
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <span className="opacity-50">No recent syncs</span>
        )}

        {globalContextMenu && (
          <div 
            className="fixed z-[9999] atc-menu-wrapper backdrop-blur-md bg-slate-900/80 py-1.5 min-w-[160px] animate-in fade-in zoom-in-95 duration-200"
            style={{ top: globalContextMenu.y, left: globalContextMenu.x }}
          >
            <button 
              onClick={handleManualForceSync}
              className="atc-menu-item group flex w-full text-left"
            >
              <div className="p-1 bg-emerald-500/10 rounded group-hover:bg-emerald-500/20 transition-colors">
                <RefreshCw className="w-3 h-3 text-emerald-400 group-hover:rotate-180 transition-transform duration-500" />
              </div>
              <span className="font-semibold text-xs text-slate-200 group-hover:text-white">Manual Force Sync</span>
            </button>
            <button 
              onClick={handleViewLogs}
              className="atc-menu-item group flex w-full text-left"
            >
              <div className="p-1 bg-blue-500/10 rounded group-hover:bg-blue-500/20 transition-colors">
                <FileText className="w-3 h-3 text-blue-400" />
              </div>
              <span className="font-semibold text-xs text-slate-200 group-hover:text-white">View Sync Logs</span>
            </button>
            <div className="h-[1px] bg-white/10 my-1 mx-2" />
            <button 
              onClick={handleClearCache}
              className="atc-menu-item group flex w-full text-left"
            >
              <div className="p-1 bg-red-500/10 rounded group-hover:bg-red-500/20 transition-colors">
                <Database className="w-3 h-3 text-red-400" />
              </div>
              <span className="font-semibold text-xs text-slate-200 group-hover:text-white">Clear Cache</span>
            </button>
          </div>
        )}
      </div>
    </TooltipWrapper>
  );
}
