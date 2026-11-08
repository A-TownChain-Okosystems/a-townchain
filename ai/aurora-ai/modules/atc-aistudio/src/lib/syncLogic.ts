// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export type SyncItem = {
  id: string;
  target: 'GitHub' | 'Notion' | 'System';
  payload: any;
  retryCount: number;
};

export class SyncManager {
  private queue: SyncItem[] = [];

  constructor(initialQueue: SyncItem[] = []) {
    this.queue = initialQueue;
  }

  enqueue(item: Omit<SyncItem, 'id' | 'retryCount'>): SyncItem {
    const newItem: SyncItem = {
      ...item,
      id: Math.random().toString(36).substring(7),
      retryCount: 0,
    };
    this.queue.push(newItem);
    return newItem;
  }

  getQueue() {
    return this.queue;
  }

  async processSync(
    isOnline: boolean,
    syncFn: (item: SyncItem) => Promise<boolean>
  ): Promise<{ processed: number; failed: number }> {
    if (!isOnline) {
      return { processed: 0, failed: this.queue.length };
    }

    let processed = 0;
    let failed = 0;
    const remainingQueue: SyncItem[] = [];

    for (const item of this.queue) {
      if (item.retryCount >= 3) {
        // Drop after 3 retries
        failed++;
        continue;
      }

      try {
        const success = await syncFn(item);
        if (success) {
          processed++;
        } else {
          item.retryCount++;
          failed++;
          remainingQueue.push(item);
        }
      } catch (e) {
        item.retryCount++;
        failed++;
        remainingQueue.push(item);
      }
    }

    this.queue = remainingQueue;
    return { processed, failed };
  }
}
