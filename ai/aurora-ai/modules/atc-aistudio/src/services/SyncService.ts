// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import { ROADMAP_DATA } from '../roadmapData';
import { WIKI_CONTENT } from '../wikiData';

export interface SyncStatus {
  status: 'idle' | 'syncing' | 'success' | 'error';
  lastSync: number | null;
  message: string;
}

class UnifiedSyncService {
  private static instance: UnifiedSyncService;
  private listeners: ((status: SyncStatus) => void)[] = [];
  
  private currentStatus: SyncStatus = {
    status: 'idle',
    lastSync: null,
    message: 'Ready to sync'
  };

  private constructor() {}

  public static getInstance(): UnifiedSyncService {
    if (!UnifiedSyncService.instance) {
      UnifiedSyncService.instance = new UnifiedSyncService();
    }
    return UnifiedSyncService.instance;
  }

  public subscribe(listener: (status: SyncStatus) => void) {
    this.listeners.push(listener);
    listener(this.currentStatus);
    return () => {
      this.listeners = this.listeners.filter(l => l !== listener);
    };
  }

  private notify() {
    this.listeners.forEach(l => l(this.currentStatus));
  }

  private setStatus(update: Partial<SyncStatus>) {
    this.currentStatus = { ...this.currentStatus, ...update };
    this.notify();
  }

  public async synchronize() {
    if (this.currentStatus.status === 'syncing') return;
    
    this.setStatus({ status: 'syncing', message: 'Starting unified synchronization...' });
    
    try {
      // 1. Fetch from Roadmap
      this.setStatus({ message: 'Fetching ATC Roadmap data...' });
      await new Promise(resolve => setTimeout(resolve, 600));
      const roadmapItems = ROADMAP_DATA;
      
      // 2. Fetch from Wiki
      this.setStatus({ message: 'Fetching Knowledge Wiki documentation...' });
      await new Promise(resolve => setTimeout(resolve, 800));
      const wikiItems = WIKI_CONTENT;
      
      // 3. Update internal local knowledge DB (Simulated by updating localStorage or state)
      this.setStatus({ message: 'Updating local Knowledge Database...' });
      
      const combinedDocs = [
        ...roadmapItems.map(item => ({
             id: `roadmap-${item.id}`,
             type: 'roadmap',
             title: `Roadmap: ${item.title}`,
             content: `## ${item.title}\n\n**Status:** ${item.status || 'Planned'}\n**Timeframe:** ${item.timeframe}\n\n### Goals\n${item.goals?.map((g: any) => `- [${g.completed ? 'x' : ' '}] ${g.text}`).join('\n')}`
        })),
        ...wikiItems.map(item => ({
             id: `wiki-${item.id}`,
             type: 'wiki',
             title: `Wiki: ${item.title}`,
             content: `## ${item.title}\n\n**Category:** ${item.category} / ${item.subcategory}\n\n${item.text}\n\n${item.table ? '### Component Specs\n' + item.table.map((r: any) => `- **${r.component}**: ${r.desc} (${r.status})`).join('\n') : ''}`
        }))
      ];

      localStorage.setItem('atc_unified_knowledge_sync', JSON.stringify(combinedDocs));

      // Trigger a global custom event for other components to re-fetch
      window.dispatchEvent(new CustomEvent('atc-knowledge-synced', { detail: { count: combinedDocs.length } }));

      await new Promise(resolve => setTimeout(resolve, 400));
      this.setStatus({ 
        status: 'success', 
        message: `Successfully synchronized ${combinedDocs.length} documentation artifacts.`,
        lastSync: Date.now()
      });

      // Reset to idle after 3s
      setTimeout(() => {
        if (this.currentStatus.status === 'success') {
          this.setStatus({ status: 'idle', message: 'Ready to sync' });
        }
      }, 3000);

    } catch (error: any) {
      this.setStatus({ status: 'error', message: `Sync failed: ${error.message}` });
    }
  }
}

export const syncService = UnifiedSyncService.getInstance();
