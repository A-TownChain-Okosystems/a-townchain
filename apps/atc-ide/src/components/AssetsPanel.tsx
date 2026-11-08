import React, { useState } from 'react';
import { Image, Upload, FileAudio, FileVideo, FileText, Trash2, Link } from 'lucide-react';

interface Asset {
  id: string;
  name: string;
  type: 'image' | 'audio' | 'video' | 'text';
  size: string;
  url: string;
}

export function AssetsPanel() {
  const [assets, setAssets] = useState<Asset[]>([
    { id: '1', name: 'hero-background.jpg', type: 'image', size: '2.4 MB', url: '#' },
    { id: '2', name: 'jump-sound.wav', type: 'audio', size: '124 KB', url: '#' },
    { id: '3', name: 'ambient-loop.mp3', type: 'audio', size: '3.1 MB', url: '#' },
    { id: '4', name: 'enemy-sprite.png', type: 'image', size: '45 KB', url: '#' },
    { id: '5', name: 'level-1-data.json', type: 'text', size: '12 KB', url: '#' }
  ]);

  const getIcon = (type: string) => {
    switch (type) {
      case 'image': return <Image className="w-8 h-8 text-cyan-400 opacity-50" />;
      case 'audio': return <FileAudio className="w-8 h-8 text-emerald-400 opacity-50" />;
      case 'video': return <FileVideo className="w-8 h-8 text-purple-400 opacity-50" />;
      default: return <FileText className="w-8 h-8 text-slate-400 opacity-50" />;
    }
  };

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0c0c0e]">
      <div className="p-6 border-b border-white/10 flex items-center justify-between bg-black/20">
        <div>
          <h2 className="text-xl font-bold text-slate-200">Projekt Assets</h2>
          <p className="text-sm text-slate-500 mt-1">Verwalte Bilder, Sounds und andere statische Dateien für dein Projekt.</p>
        </div>
        <button className="flex items-center gap-2 bg-cyan-600 hover:bg-cyan-500 text-white px-4 py-2 rounded-lg font-bold transition-colors">
          <Upload className="w-4 h-4" />
          Asset hochladen
        </button>
      </div>

      <div className="p-6 flex-1 overflow-y-auto">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {assets.map(asset => (
            <div key={asset.id} className="bg-white/5 border border-white/10 rounded-xl overflow-hidden group hover:border-cyan-500/50 transition-colors">
              <div className="h-32 bg-black/40 flex items-center justify-center relative">
                {getIcon(asset.type)}
                <div className="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center gap-2">
                   <button title="URL kopieren" className="p-2 bg-white/10 hover:bg-cyan-500/50 text-white rounded-full transition-colors">
                     <Link className="w-4 h-4" />
                   </button>
                   <button title="Löschen" className="p-2 bg-white/10 hover:bg-red-500/50 text-white rounded-full transition-colors">
                     <Trash2 className="w-4 h-4" />
                   </button>
                </div>
              </div>
              <div className="p-3 border-t border-white/10">
                <div className="text-sm font-bold text-slate-200 truncate" title={asset.name}>{asset.name}</div>
                <div className="text-xs text-slate-500 mt-0.5 flex justify-between">
                  <span className="uppercase">{asset.type}</span>
                  <span>{asset.size}</span>
                </div>
              </div>
            </div>
          ))}

          {/* Upload Dropzone */}
          <div className="border-2 border-dashed border-white/10 hover:border-cyan-500/50 rounded-xl h-[190px] flex flex-col items-center justify-center text-slate-500 hover:text-cyan-400 transition-colors cursor-pointer bg-white/[0.02] hover:bg-cyan-500/5">
             <Upload className="w-8 h-8 mb-2" />
             <span className="text-sm font-bold">Datei ablegen</span>
             <span className="text-xs opacity-70">oder klicken</span>
          </div>
        </div>
      </div>
    </div>
  );
}
