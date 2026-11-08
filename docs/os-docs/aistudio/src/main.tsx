import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import DesktopApp from './DesktopApp.tsx';
import { WalletProvider } from './contexts/WalletContext.tsx';
import { GoogleWorkspaceProvider } from './contexts/GoogleWorkspaceContext.tsx';
import { FirebaseProvider } from './contexts/FirebaseContext.tsx';
import { SyncMetricsProvider } from './contexts/SyncMetricsContext.tsx';
import './index.css';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <FirebaseProvider>
      <GoogleWorkspaceProvider>
        <SyncMetricsProvider>
          <WalletProvider>
            <DesktopApp />
          </WalletProvider>
        </SyncMetricsProvider>
      </GoogleWorkspaceProvider>
    </FirebaseProvider>
  </StrictMode>,
);


