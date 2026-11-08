import React, { createContext, useContext, ReactNode } from 'react';
import { useGoogleSheetsSync } from '../hooks/useGoogleSheetsSync';

interface SyncMetricsContextType {
  isSyncing: boolean;
  lastSyncTime: Date | null;
  syncHistory: { service: 'drive' | 'sheets' | 'docs' | 'calendar'; timestamp: string; status: 'success' | 'error'; latency: number }[];
  serviceSyncTimes: {
    drive: Date | null;
    sheets: Date | null;
    docs: Date | null;
    calendar: Date | null;
  };
  syncProgress: {
    drive?: number;
    sheets?: number;
    docs?: number;
    calendar?: number;
  };
  forceSync: (service?: string) => Promise<void>;
}

const SyncMetricsContext = createContext<SyncMetricsContextType>({
  isSyncing: false,
  lastSyncTime: null,
  syncHistory: [],
  serviceSyncTimes: {
    drive: null,
    sheets: null,
    docs: null,
    calendar: null,
  },
  syncProgress: {},
  forceSync: async () => {}
});

export const SyncMetricsProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const syncState = useGoogleSheetsSync(60000);

  return (
    <SyncMetricsContext.Provider value={syncState}>
      {children}
    </SyncMetricsContext.Provider>
  );
};

export const useSyncMetrics = () => useContext(SyncMetricsContext);
