import { WINDOWS_MAP } from '../DesktopApp.tsx';
import { MARKETPLACE_EXTRA_APPS } from '../marketplaceApps.ts';
import { Package } from 'lucide-react';

export function syncInstalledApps() {
  try {
    const currentApps = JSON.parse(localStorage.getItem('atc_user_apps') || 'null');
    
    // Auto-install all default apps once for convenience if empty
    let appsToProcess = currentApps;
    if (!appsToProcess || appsToProcess.length === 0) {
      appsToProcess = MARKETPLACE_EXTRA_APPS.map(app => ({
        ...app,
        isRealApp: true,
        purchasedAt: new Date().toISOString()
      }));
      localStorage.setItem('atc_user_apps', JSON.stringify(appsToProcess));
    }

    let updated = false;
    const catMap: Record<string, string> = {
      "Productivity": "dokumente",
      "Media & Web": "medien",
      "Design": "medien",
      "Gaming": "gaming",
      "Social Network": "sozial",
      "AI Publishing": "ki",
      "Utilities": "system",
      "Developer Tools": "system"
    };

    appsToProcess.forEach((asset: any) => {
      if (asset.isRealApp) {
        const match = MARKETPLACE_EXTRA_APPS.find(app => app.id === asset.id);
        if (match && !WINDOWS_MAP[match.id]) {
          WINDOWS_MAP[match.id] = {
            label: match.name,
            icon: match.icon,
            category: catMap[match.category] || "system",
            component: match.component,
          };
          updated = true;
        }
      } else {
        // Dummy apps
        const newWinId = `custom_${asset.id}`.replace(/[^a-zA-Z0-9_]/g, '_');
        if (!WINDOWS_MAP[newWinId]) {
          WINDOWS_MAP[newWinId] = {
            label: asset.name,
            icon: Package,
            category: "system",
            component: () => (
              <div className="flex flex-col items-center justify-center h-full bg-[#050811] text-white">
                <Package className="w-16 h-16 text-indigo-400 mb-4" />
                <h2 className="text-2xl font-bold">{asset.name}</h2>
                <p className="text-slate-400 mt-2 text-sm italic">This module was installed from the ATC Marketplace.</p>
              </div>
            )
          };
          updated = true;
        }
      }
    });

    return updated; // Returns true if WINDOWS_MAP was updated
  } catch (err) {
    console.error("Failed to sync installed apps", err);
    return false;
  }
}

export function installApplet(selectedItem: any) {
  const currentApps = JSON.parse(localStorage.getItem('atc_user_apps') || '[]');
  currentApps.push({ ...selectedItem, purchasedAt: new Date().toISOString() });
  localStorage.setItem('atc_user_apps', JSON.stringify(currentApps));
  syncInstalledApps();
}

export function installApplets(selectedItems: any[]) {
  const currentApps = JSON.parse(localStorage.getItem('atc_user_apps') || '[]');
  const newApps = selectedItems.map(item => ({ ...item, purchasedAt: new Date().toISOString() }));
  localStorage.setItem('atc_user_apps', JSON.stringify([...currentApps, ...newApps]));
  syncInstalledApps();
}
