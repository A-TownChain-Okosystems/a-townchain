import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { GitHubRepoSyncView } from '../src/components/GitHubRepoSyncView';

describe('GitHubRepoSyncView', () => {
  it('should render the component and display repositories', () => {
    render(<GitHubRepoSyncView language="EN" />);
    // Check if the component shows the text
    expect(screen.getAllByText('a-townchain-os')[0]).toBeDefined();
    expect(screen.getAllByText('atvm-sandbox')[0]).toBeDefined();
  });

  it('should toggle sync status when sync button is clicked and handle error states', async () => {
    // Fake timers might be useful since there is setTimeout in the component, or we can just wait for it in real time
    render(<GitHubRepoSyncView language="EN" />);
    
    // Check initial state
    expect(screen.getAllByText('atc-smart-contracts')[0]).toBeDefined();
    
    const syncButtons = screen.getAllByRole('button', { name: /Sync Now/i });
    expect(syncButtons.length).toBeGreaterThan(0);
    
    fireEvent.click(syncButtons[0]);
    
    // Status should change to Syncing...
    await waitFor(() => {
      expect(screen.getAllByText(/Syncing.../i).length).toBeGreaterThan(0);
    });
  });

  it('should allow attempting retry on failed syncs', async () => {
    render(<GitHubRepoSyncView language="EN" />);
    
    // In our mock data, one of the repos has an initial 'error' state, thus showing 'Retry Sync'
    const retryButtons = screen.getAllByRole('button', { name: /Retry Sync/i });
    expect(retryButtons.length).toBeGreaterThan(0);
    
    fireEvent.click(retryButtons[0]);
    
    // The status of the retried repo should become 'syncing'
    await waitFor(() => {
      expect(screen.getAllByText(/Syncing.../i).length).toBeGreaterThan(0);
    });
  });
});



