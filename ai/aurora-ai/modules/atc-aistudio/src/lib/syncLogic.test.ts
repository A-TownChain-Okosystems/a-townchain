// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import { describe, it, expect, vi } from 'vitest';
import { SyncManager } from './syncLogic';

describe('SyncManager Core Logic', () => {
  it('should enqueue items properly', () => {
    const manager = new SyncManager();
    const item = manager.enqueue({ target: 'GitHub', payload: { data: 123 } });
    
    expect(item.id).toBeDefined();
    expect(item.retryCount).toBe(0);
    expect(item.target).toBe('GitHub');
    expect(manager.getQueue().length).toBe(1);
  });

  it('should process sync successfully when online and clear the queue', async () => {
    const manager = new SyncManager();
    manager.enqueue({ target: 'GitHub', payload: {} });
    manager.enqueue({ target: 'Notion', payload: {} });

    const mockSyncFn = vi.fn().mockResolvedValue(true);
    
    const result = await manager.processSync(true, mockSyncFn);
    
    expect(result.processed).toBe(2);
    expect(result.failed).toBe(0);
    expect(manager.getQueue().length).toBe(0);
    expect(mockSyncFn).toHaveBeenCalledTimes(2);
  });

  it('should not process and keep queue when offline', async () => {
    const manager = new SyncManager();
    manager.enqueue({ target: 'GitHub', payload: {} });

    const mockSyncFn = vi.fn();
    const result = await manager.processSync(false, mockSyncFn);
    
    expect(result.processed).toBe(0);
    expect(result.failed).toBe(1); // entire queue "failed" to process
    expect(manager.getQueue().length).toBe(1);
    expect(mockSyncFn).not.toHaveBeenCalled();
  });

  it('should keep items in queue and increment retry count on failure', async () => {
    const manager = new SyncManager();
    manager.enqueue({ target: 'GitHub', payload: {} });

    // Mock failure
    const mockSyncFn = vi.fn().mockResolvedValue(false);
    const result = await manager.processSync(true, mockSyncFn);
    
    expect(result.processed).toBe(0);
    expect(result.failed).toBe(1);
    
    const queue = manager.getQueue();
    expect(queue.length).toBe(1);
    expect(queue[0].retryCount).toBe(1);
  });

  it('should drop items from queue after 3 retries', async () => {
    const manager = new SyncManager();
    manager.enqueue({ target: 'GitHub', payload: {} });
    const mockSyncFn = vi.fn().mockResolvedValue(false);

    // 1st attempt
    await manager.processSync(true, mockSyncFn);
    expect(manager.getQueue()[0].retryCount).toBe(1);

    // 2nd attempt
    await manager.processSync(true, mockSyncFn);
    expect(manager.getQueue()[0].retryCount).toBe(2);

    // 3rd attempt
    await manager.processSync(true, mockSyncFn);
    expect(manager.getQueue()[0].retryCount).toBe(3);

    // 4th attempt - should drop the item
    const finalResult = await manager.processSync(true, mockSyncFn);
    expect(finalResult.failed).toBe(1); 
    expect(manager.getQueue().length).toBe(0); // Dropped
  });
});
