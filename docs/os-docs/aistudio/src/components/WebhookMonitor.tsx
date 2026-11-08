import React, { useState, useEffect } from 'react';
import { Activity, CheckCircle, XCircle, Clock } from 'lucide-react';
import { ATC_OS_REQUIREMENTS } from '../requirementsData';

export interface WebhookStatus {
  repoName: string;
  owner: string;
  status: 'loading' | 'success' | 'error' | 'unauthorized';
  lastPolled: Date | null;
  message?: string;
}

const REPOSITORIES = ATC_OS_REQUIREMENTS.map(req => ({
  name: `atc-${req.id}`,
  owner: 'atown-chain'
}));


interface WebhookMonitorProps {
  onStatusesUpdate?: (statuses: WebhookStatus[]) => void;
}

export function WebhookMonitor({ onStatusesUpdate }: WebhookMonitorProps = {}) {
  const [statuses, setStatuses] = useState<WebhookStatus[]>(
    REPOSITORIES.map(repo => ({
      repoName: repo.name,
      owner: repo.owner,
      status: 'loading',
      lastPolled: null
    }))
  );

  const pollGitHubWebhooks = async () => {
    const updatedStatuses = await Promise.all(
      REPOSITORIES.map(async (repo) => {
        try {
          // Attempting to fetch webhooks for the repository
          // Without an auth token, this will likely return 401 Unauthorized or 404 Not Found
          const response = await fetch(`https://api.github.com/repos/${repo.owner}/${repo.name}/hooks`);
          
          if (response.ok) {
            return {
              repoName: repo.name,
              owner: repo.owner,
              status: 'success' as const,
              lastPolled: new Date(),
              message: '200 OK'
            };
          } else if (response.status === 401 || response.status === 403 || response.status === 404) {
            // Mock a success status for the dashboard if unauthorized (as requested to check/simulate)
            // But let's actually report the response
            return {
              repoName: repo.name,
              owner: repo.owner,
              status: 'error' as const, // Or unauthorized
              lastPolled: new Date(),
              message: `${response.status} ${response.statusText}`
            };
          }
          
          return {
            repoName: repo.name,
            owner: repo.owner,
            status: 'error' as const,
            lastPolled: new Date(),
            message: `Error ${response.status}`
          };
        } catch (error: any) {
          return {
            repoName: repo.name,
            owner: repo.owner,
            status: 'error' as const,
            lastPolled: new Date(),
            message: error.message || 'Network Error'
          };
        }
      })
    );

    // To prevent everything from always showing errors due to missing tokens in the preview environment,
    // we can fallback to displaying simulated 200 OK after checking if the real request failed for demo purposes.
    // However, the instructions say "polls the GitHub API... to check if webhook payloads are returning 200 OK"
    
    // We will display the real response.
    setStatuses(updatedStatuses);
    if (onStatusesUpdate) {
      onStatusesUpdate(updatedStatuses);
    }
  };

  useEffect(() => {
    // Initial poll
    pollGitHubWebhooks();

    // Poll every 30 seconds
    const interval = setInterval(pollGitHubWebhooks, 30000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#0b0f19] border border-white/10 rounded-2xl p-6 shadow-2xl flex flex-col gap-4 h-[400px]">
      <div className="flex items-center justify-between border-b border-white/10 pb-4 shrink-0">
         <h2 className="text-xl font-mono text-white flex items-center gap-2">
           <Activity className="w-5 h-5 text-indigo-400" />
           GitHub Webhook Monitor
         </h2>
         <span className="text-xs font-mono text-slate-400 flex items-center gap-1">
           <Clock className="w-3 h-3" />
           Polling live...
         </span>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 overflow-y-auto custom-scrollbar pr-2 pb-2">
        {statuses.map(status => (
          <div key={`${status.owner}/${status.repoName}`} className="bg-black/40 border border-white/5 rounded-xl p-4 flex flex-col gap-2 relative overflow-hidden">
            <div className="flex items-center justify-between">
              <span className="text-sm font-mono text-slate-300">{status.owner}/{status.repoName}</span>
              {status.status === 'loading' && <div className="w-4 h-4 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>}
              {status.status === 'success' && <CheckCircle className="w-4 h-4 text-emerald-500" />}
              {status.status === 'error' && <XCircle className="w-4 h-4 text-rose-500" />}
            </div>
            
            <div className="flex items-center justify-between mt-2">
              <span className="text-xs font-mono px-2 py-1 rounded-md bg-white/5 text-slate-400">
                Payload Status:
                <span className={`ml-2 font-bold ${
                  status.status === 'success' ? 'text-emerald-400' : 
                  status.status === 'error' ? 'text-rose-400' : 'text-slate-500'
                }`}>
                  {status.status === 'loading' ? 'Checking...' : status.message}
                </span>
              </span>
              <span className="text-[10px] text-slate-500 font-mono">
                {status.lastPolled ? status.lastPolled.toLocaleTimeString() : '--:--'}
              </span>
            </div>
            
            {status.status === 'error' && status.message?.includes('404') && (
              <div className="text-[10px] text-amber-500/80 mt-1">Requires authentication or repo is private / non-existent.</div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
