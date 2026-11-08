import React, { useEffect, useState } from 'react';
import { Palette, Clock } from 'lucide-react';

const THEMES = [
  { id: 'system-sync', label: 'System Sync (Auto)' },
  { id: 'oceanic-deep', label: 'Oceanic Deep' },
  { id: 'sunset-red', label: 'Sunset Red' },
  { id: 'cyber-cyan', label: 'Cyber Cyan' },
  { id: 'default', label: 'Default Dark' }
];

export function ThemeSwitcher() {
  const [theme, setTheme] = useState(() => {
    return localStorage.getItem('atc_theme') || 'default';
  });
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    let activeTheme = theme;
    
    if (theme === 'system-sync') {
      const hour = new Date().getHours();
      // Evening hours between 18:00 and 06:00
      activeTheme = (hour >= 18 || hour < 6) ? 'cyber-cyan' : 'default';
    }

    if (activeTheme === 'default') {
      document.documentElement.removeAttribute('data-theme');
    } else {
      document.documentElement.setAttribute('data-theme', activeTheme);
    }
    
    localStorage.setItem('atc_theme', theme);
    window.dispatchEvent(new Event('theme_changed_external'));
  }, [theme]);

  // Periodic check for system-sync theme
  useEffect(() => {
    if (theme !== 'system-sync') return;
    
    const interval = setInterval(() => {
      const hour = new Date().getHours();
      const activeTheme = (hour >= 18 || hour < 6) ? 'cyber-cyan' : 'default';
      
      if (activeTheme === 'default') {
        document.documentElement.removeAttribute('data-theme');
      } else {
        document.documentElement.setAttribute('data-theme', activeTheme);
      }
    }, 60000); // Check every minute
    
    return () => clearInterval(interval);
  }, [theme]);

  useEffect(() => {
    const handleSync = () => {
      const stored = localStorage.getItem('atc_theme') || 'default';
      setTheme(stored);
    };
    window.addEventListener('storage', handleSync);
    window.addEventListener('theme_changed', handleSync);
    return () => {
      window.removeEventListener('storage', handleSync);
      window.removeEventListener('theme_changed', handleSync);
    };
  }, []);

  // Click outside listener
  useEffect(() => {
    if (!isOpen) return;
    const handleClick = (e: MouseEvent) => {
      if (!(e.target as Element).closest('.theme-switcher-container')) {
        setIsOpen(false);
      }
    };
    document.addEventListener('click', handleClick);
    return () => document.removeEventListener('click', handleClick);
  }, [isOpen]);

  const handleHover = (themeId: string) => {
    let activeTheme = themeId;
    if (themeId === 'system-sync') {
      const hour = new Date().getHours();
      activeTheme = (hour >= 18 || hour < 6) ? 'cyber-cyan' : 'default';
    }
    
    if (activeTheme === 'default') {
      document.documentElement.removeAttribute('data-theme');
    } else {
      document.documentElement.setAttribute('data-theme', activeTheme);
    }
  };

  const handleHoverLeave = () => {
    let activeTheme = theme;
    if (theme === 'system-sync') {
      const hour = new Date().getHours();
      activeTheme = (hour >= 18 || hour < 6) ? 'cyber-cyan' : 'default';
    }

    if (activeTheme === 'default') {
      document.documentElement.removeAttribute('data-theme');
    } else {
      document.documentElement.setAttribute('data-theme', activeTheme);
    }
  };

  return (
    <div className="relative theme-switcher-container">
      <button 
        onClick={() => setIsOpen(!isOpen)}
        className="p-2 text-slate-500 hover:text-atc-cyan hover:bg-atc-purple/10 rounded-full transition-colors flex items-center gap-2"
        title="Toggle Theme"
      >
        <Palette className="w-4 h-4" />
      </button>

      {isOpen && (
        <div className="absolute right-0 mt-2 w-48 bg-[#090b14] border border-white/10 rounded-xl shadow-2xl overflow-hidden z-50">
          <div className="px-3 py-2 border-b border-white/10">
             <span className="text-xs font-mono font-bold uppercase tracking-wider text-slate-400">Select Theme</span>
          </div>
          <div className="flex flex-col py-1" onMouseLeave={handleHoverLeave}>
            {THEMES.map(t => (
              <button
                key={t.id}
                onMouseEnter={() => handleHover(t.id)}
                onClick={() => {
                  setTheme(t.id);
                  setIsOpen(false);
                }}
                className={`flex items-center gap-2 text-left px-4 py-2 text-sm transition-colors ${theme === t.id ? 'bg-atc-cyan/10 text-atc-cyan font-medium' : 'text-slate-300 hover:bg-white/5'}`}
              >
                {t.id === 'system-sync' && <Clock className="w-3.5 h-3.5" />}
                {t.label}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
