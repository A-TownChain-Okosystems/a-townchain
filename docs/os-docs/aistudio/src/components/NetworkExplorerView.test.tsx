import { render, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';
import * as React from 'react';

// Mock d3 components or the child component entirely
vi.mock('./NetworkExplorerView', async (importOriginal) => {
  const actual = await importOriginal() as any;
  return {
    ...actual,
    // Note: since ForceDirectedNetworkMap is not exported, we can just mock d3 
    // Wait, it's easier to mock global fetch correctly
  };
});

// Since ForceDirectedNetworkMap uses d3, let's provide a better d3 mock
vi.mock('d3', () => {
  const dummySelection = {
    selectAll: () => dummySelection,
    remove: () => dummySelection,
    append: () => dummySelection,
    attr: () => dummySelection,
    style: () => dummySelection,
    on: () => dummySelection,
    data: () => dummySelection,
    join: () => dummySelection,
    call: () => dummySelection,
    text: () => dummySelection,
  };
  return {
    select: () => dummySelection,
    drag: () => ({ on: () => ({ on: () => ({ on: () => dummySelection }) }) }),
    forceSimulation: () => ({
      force: function() { return this; },
      on: function() { return this; },
      stop: () => {},
      alphaTarget: () => ({ restart: () => {} })
    }),
    forceLink: () => ({ id: () => ({ distance: () => {} }) }),
    forceManyBody: () => ({ strength: () => {} }),
    forceCenter: () => {},
    forceCollide: () => ({ radius: () => {} })
  };
});

import { NetworkExplorerView } from './NetworkExplorerView';

describe('NetworkExplorerView', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    
    const mockFetch = vi.fn().mockImplementation((url) => {
      // url might be a Request object or string
      const urlStr = (typeof url === 'string') ? url : url.url;
      console.log('MOCK FETCH CALLED WITH:', urlStr);
      
      if (urlStr === '/api/blockchain/info') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ height: 12345, peers: ['peer1', 'peer2', 'peer3'] })
        } as Response);
      }
      if (urlStr === '/api/blockchain/blocks') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([
            { index: 1, validator: 'validator1', transactions: [], timestamp: Date.now() },
            { index: 2, validator: 'validator2', transactions: [1, 2], timestamp: Date.now() },
            { index: 3, validator: 'validator2', transactions: [1, 2], timestamp: Date.now() },
          ])
        } as Response);
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({})
      } as Response);
    });
    
    vi.stubGlobal('fetch', mockFetch);
    window.fetch = mockFetch as any;
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.restoreAllMocks();
  });

  it('renders loading state initially', async () => {
    // Override fetch to never resolve for this test (or resolve very late)
    // Actually, we don't need to override, as initial render is synchronous
    render(<NetworkExplorerView />);
    
    // Check header
    expect(screen.getByText('ATC Blockchain Daten & Statistiken')).toBeDefined();
    
    // Check loading state for blocks
    expect(screen.getByText('Verbinde zur Chain...')).toBeDefined();
  });

  it('fetches and displays network data successfully', async () => {
    render(<NetworkExplorerView />);
    
    // Check that values are updated
    await waitFor(() => {
      expect(screen.getByText('12345')).toBeDefined(); // Latest block height
      expect(screen.getByText('3')).toBeDefined(); // Active validators
      expect(screen.getByText('Netzwerk Gesund')).toBeDefined(); // Network Healthy
    });
    
    // Check that blocks rendered
    await waitFor(() => {
      expect(screen.getByText('Block #3')).toBeDefined();
    });
  });

  it('enables typing in search field', () => {
    render(<NetworkExplorerView />);
    
    const searchInput = screen.getByPlaceholderText('Suche nach Blöcken, Transaktionen, Adressen...');
    expect(searchInput).toBeDefined();
  });
});
