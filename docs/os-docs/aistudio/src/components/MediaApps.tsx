import React, { useState } from 'react';
import { Play, Pause, SkipBack, SkipForward, Volume2, Image as ImageIcon, Music, Video, List, Maximize, Settings, FileText, Heart, Repeat, Shuffle, RefreshCw, Minus, Plus } from 'lucide-react';

export function ATCMediaPlayerHub() {
  const [activeTab, setActiveTab] = useState<'all' | 'audio' | 'video' | 'images'>('all');

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="flex items-center gap-4 p-4 border-b border-white/5 bg-black/40">
        <div className="flex bg-white/5 p-1 rounded-lg">
          <button onClick={() => setActiveTab('all')} className={`px-4 py-1.5 rounded-md text-sm transition-colors ${activeTab === 'all' ? 'bg-indigo-500/20 text-indigo-300' : 'hover:bg-white/5 text-slate-400'}`}>Übersicht</button>
          <button onClick={() => setActiveTab('audio')} className={`flex items-center gap-2 px-4 py-1.5 rounded-md text-sm transition-colors ${activeTab === 'audio' ? 'bg-emerald-500/20 text-emerald-300' : 'hover:bg-white/5 text-slate-400'}`}><Music className="w-4 h-4" /> Audio</button>
          <button onClick={() => setActiveTab('video')} className={`flex items-center gap-2 px-4 py-1.5 rounded-md text-sm transition-colors ${activeTab === 'video' ? 'bg-rose-500/20 text-rose-300' : 'hover:bg-white/5 text-slate-400'}`}><Video className="w-4 h-4" /> Video</button>
          <button onClick={() => setActiveTab('images')} className={`flex items-center gap-2 px-4 py-1.5 rounded-md text-sm transition-colors ${activeTab === 'images' ? 'bg-cyan-500/20 text-cyan-300' : 'hover:bg-white/5 text-slate-400'}`}><ImageIcon className="w-4 h-4" /> Bilder</button>
        </div>
      </div>
      <div className="flex-1 p-6 overflow-y-auto">
        {/* Hub content: showing recent files or collections */}
        {(activeTab === 'all' || activeTab === 'audio') && (
        <div className="mb-8">
           <div className="flex items-center justify-between mb-4">
               <h2 className="text-lg font-bold text-white flex items-center gap-2"><Music className="w-5 h-5 text-emerald-400" /> Audio Bibliothek</h2>
           </div>
           <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <div onClick={() => window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'atc_audio' }))} className="bg-white/5 border border-white/10 rounded-xl p-4 hover:border-emerald-500/30 transition-colors cursor-pointer group">
                 <div className="w-full aspect-square bg-emerald-500/10 rounded-lg flex items-center justify-center mb-4 relative overflow-hidden">
                    <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1614613535308-eb5fbd3d2c17?w=500&q=80')] bg-cover bg-center opacity-40 mix-blend-overlay group-hover:opacity-60 transition-opacity"></div>
                    <Music className="w-10 h-10 text-emerald-500/50 group-hover:text-emerald-400 transition-colors relative z-10" />
                 </div>
                 <h3 className="font-bold text-white text-sm truncate">Vaporwave Memories</h3>
                 <p className="text-xs text-slate-400 mt-1 truncate">A-Town Syntax</p>
              </div>
              <div onClick={() => window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'atc_audio' }))} className="bg-white/5 border border-white/10 rounded-xl p-4 hover:border-emerald-500/30 transition-colors cursor-pointer group">
                 <div className="w-full aspect-square bg-emerald-500/10 rounded-lg flex items-center justify-center mb-4 relative overflow-hidden">
                    <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1470225620780-dba8ba36b745?w=500&q=80')] bg-cover bg-center opacity-40 mix-blend-overlay group-hover:opacity-60 transition-opacity"></div>
                    <Music className="w-10 h-10 text-emerald-500/50 group-hover:text-emerald-400 transition-colors relative z-10" />
                 </div>
                 <h3 className="font-bold text-white text-sm truncate">Midnight Chords</h3>
                 <p className="text-xs text-slate-400 mt-1 truncate">DJ Null Pointer</p>
              </div>
           </div>
        </div>
        )}

        {(activeTab === 'all' || activeTab === 'video') && (
        <div className="mb-8">
           <div className="flex items-center justify-between mb-4">
               <h2 className="text-lg font-bold text-white flex items-center gap-2"><Video className="w-5 h-5 text-rose-400" /> Video Sammlung</h2>
           </div>
           <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
              <div onClick={() => window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'atc_video' }))} className="bg-white/5 border border-white/10 rounded-xl p-4 hover:border-rose-500/30 transition-colors cursor-pointer group">
                 <div className="w-full aspect-video bg-rose-500/10 rounded-lg flex items-center justify-center mb-4 relative overflow-hidden">
                    <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1536440136628-849c177e76a1?w=500&q=80')] bg-cover bg-center opacity-30 group-hover:opacity-50 transition-opacity mix-blend-luminosity"></div>
                    <Play className="w-10 h-10 text-rose-500/50 group-hover:text-rose-400 transition-colors relative z-10" />
                 </div>
                 <h3 className="font-bold text-white text-sm truncate">A-Town Cinematic Engine</h3>
                 <p className="text-xs text-slate-400 mt-1">05:00 • 4K HDR</p>
              </div>
           </div>
        </div>
        )}

        {(activeTab === 'all' || activeTab === 'images') && (
        <div className="mb-0">
           <div className="flex items-center justify-between mb-4">
               <h2 className="text-lg font-bold text-white flex items-center gap-2"><ImageIcon className="w-5 h-5 text-cyan-400" /> Bilder Galerie</h2>
           </div>
           <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-4">
              {[...Array(5)].map((_, i) => (
                <div key={i} onClick={() => window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'atc_bilder' }))} className="bg-white/5 border border-white/10 rounded-xl p-3 hover:border-cyan-500/30 transition-colors cursor-pointer group">
                   <div className="w-full aspect-square bg-cyan-500/10 rounded-lg flex items-center justify-center mb-3 relative overflow-hidden ring-1 ring-white/5">
                      <img src={`https://images.unsplash.com/photo-1542038784456-1ea8e935640e?w=300&q=80&auto=format&fit=crop&sig=${i}`} className="absolute inset-0 w-full h-full object-cover opacity-70 group-hover:opacity-100 transition-opacity group-hover:scale-110 duration-500" alt="thumbnail" />
                   </div>
                   <h3 className="font-bold text-white text-xs truncate">IMG_92{14 + i}_Cyberpunk.jpg</h3>
                </div>
              ))}
           </div>
        </div>
        )}
      </div>
    </div>
  );
}

export function ATCAudioPlayerView() {
  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(30);
  const [currentTrack, setCurrentTrack] = useState(0);

  const tracks = [
    { title: "Vaporwave Memories", artist: "A-Town Syntax", duration: "04:12", img: "https://images.unsplash.com/photo-1614613535308-eb5fbd3d2c17?w=500&q=80" },
    { title: "Midnight Chords", artist: "DJ Null Pointer", duration: "03:45", img: "https://images.unsplash.com/photo-1470225620780-dba8ba36b745?w=500&q=80" },
    { title: "Cybernetic Echoes", artist: "The Hash Rates", duration: "05:30", img: "https://images.unsplash.com/photo-1518609878373-06d740f60d8b?w=500&q=80" }
  ];

  return (
    <div className="flex flex-col md:flex-row h-full bg-[#050811] text-slate-200">
       <div className="flex-1 flex items-center justify-center p-8">
          <div className="w-full max-w-sm flex flex-col items-center">
             <div className="w-48 h-48 bg-gradient-to-br from-emerald-900 to-indigo-900 rounded-2xl shadow-[0_10px_40px_rgba(16,185,129,0.2)] mb-8 flex items-center justify-center relative overflow-hidden group">
                <div className="absolute inset-0 bg-cover bg-center opacity-50 mix-blend-overlay transition-all duration-500" style={{ backgroundImage: `url('${tracks[currentTrack].img}')` }}></div>
                <Music className="w-16 h-16 text-white/50 relative z-10 drop-shadow-2xl" />
                <div className={`absolute inset-0 bg-emerald-500/10 mix-blend-color transition-opacity ${isPlaying ? 'opacity-100 animate-pulse' : 'opacity-0'}`}></div>
             </div>
             
             <div className="text-center mb-8">
                <h2 className="text-2xl font-bold text-white tracking-tight">{tracks[currentTrack].title}</h2>
                <p className="text-emerald-400 font-mono text-sm mt-1">{tracks[currentTrack].artist}</p>
             </div>
             
             <div className="w-full mb-6">
                <div className="flex justify-between text-xs font-mono text-slate-400 mb-2">
                   <span>01:24</span>
                   <span>{tracks[currentTrack].duration}</span>
                </div>
                <div className="h-1.5 bg-white/10 rounded-full overflow-hidden cursor-pointer" onClick={(e) => {
                   const rect = e.currentTarget.getBoundingClientRect();
                   const clickX = e.clientX - rect.left;
                   setProgress((clickX / rect.width) * 100);
                }}>
                   <div className="h-full bg-emerald-500 rounded-full" style={{ width: `${progress}%` }}></div>
                </div>
             </div>
             
             <div className="flex items-center justify-center gap-6 w-full">
                <button className="text-slate-400 hover:text-white transition-colors"><Shuffle className="w-5 h-5" /></button>
                <button onClick={() => { setCurrentTrack((prev) => (prev - 1 + tracks.length) % tracks.length); setProgress(0); }} className="text-slate-200 hover:text-white transition-colors"><SkipBack className="w-8 h-8 fill-current" /></button>
                <button onClick={() => setIsPlaying(!isPlaying)} className="w-14 h-14 bg-emerald-500 text-black rounded-full flex items-center justify-center hover:scale-105 transition-transform shadow-[0_0_20px_rgba(16,185,129,0.3)]">
                   {isPlaying ? <Pause className="w-6 h-6 fill-current" /> : <Play className="w-6 h-6 fill-current ml-1" />}
                </button>
                <button onClick={() => { setCurrentTrack((prev) => (prev + 1) % tracks.length); setProgress(0); }} className="text-slate-200 hover:text-white transition-colors"><SkipForward className="w-8 h-8 fill-current" /></button>
                <button className="text-slate-400 hover:text-white transition-colors"><Repeat className="w-5 h-5" /></button>
             </div>
          </div>
       </div>
       <div className="w-full md:w-80 border-t md:border-t-0 md:border-l border-white/5 bg-black/20 p-4 overflow-y-auto">
          <h3 className="text-sm font-bold text-white mb-4 flex items-center gap-2"><List className="w-4 h-4 text-emerald-400" /> Aktuelle Playlist</h3>
          <div className="flex flex-col gap-2">
             {tracks.map((track, i) => (
                <button 
                   key={i}
                   onClick={() => { setCurrentTrack(i); setProgress(0); setIsPlaying(true); }}
                   className={`flex items-center gap-3 p-2 rounded-lg text-left transition-colors ${currentTrack === i ? 'bg-emerald-500/20 border border-emerald-500/30' : 'hover:bg-white/5 border border-transparent'}`}
                >
                   <div className="w-10 h-10 rounded bg-emerald-500/10 flex items-center justify-center overflow-hidden shrink-0">
                      <img src={track.img} alt="" className="w-full h-full object-cover opacity-70" />
                   </div>
                   <div className="flex-1 overflow-hidden">
                      <div className={`text-sm font-medium truncate ${currentTrack === i ? 'text-emerald-300' : 'text-slate-200'}`}>{track.title}</div>
                      <div className="text-xs text-slate-400 font-mono truncate">{track.artist}</div>
                   </div>
                   <div className="text-xs text-slate-500 font-mono">{track.duration}</div>
                </button>
             ))}
          </div>
       </div>
    </div>
  );
}

export function ATCVideoPlayerView() {
  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(45);

  return (
    <div className="flex flex-col h-full bg-black text-slate-200 relative group overflow-hidden">
      {/* Video Content Placeholder */}
      <div className="absolute inset-0 bg-slate-900 overflow-hidden">
         <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1536440136628-849c177e76a1?w=1200&q=80')] bg-cover bg-center opacity-60"></div>
         {isPlaying && <div className="absolute inset-0 bg-blue-500/10 mix-blend-overlay animate-pulse"></div>}
      </div>
      
      {/* Controls Overlay */}
      <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/90 via-black/50 to-transparent pt-16 pb-4 px-6 translate-y-2 opacity-0 group-hover:translate-y-0 group-hover:opacity-100 transition-all duration-300">
         <div className="w-full mb-4">
            <div className="h-1 bg-white/20 rounded-full overflow-hidden cursor-pointer relative group/bar" onClick={(e) => {
               const rect = e.currentTarget.getBoundingClientRect();
               const clickX = e.clientX - rect.left;
               setProgress((clickX / rect.width) * 100);
            }}>
               <div className="h-full bg-rose-500 rounded-full" style={{ width: `${progress}%` }}></div>
               <div className="absolute top-1/2 -mt-1.5 w-3 h-3 bg-white rounded-full shadow opacity-0 group-hover/bar:opacity-100 transition-opacity" style={{ left: `calc(${progress}% - 6px)` }}></div>
            </div>
         </div>
         
         <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
               <button onClick={() => setIsPlaying(!isPlaying)} className="text-white hover:text-rose-400 transition-colors">
                  {isPlaying ? <Pause className="w-6 h-6 fill-current" /> : <Play className="w-6 h-6 fill-current" />}
               </button>
               <div className="flex items-center gap-2 text-white">
                  <Volume2 className="w-5 h-5" />
                  <div className="w-16 h-1 bg-white/20 rounded-full overflow-hidden">
                     <div className="w-2/3 h-full bg-white rounded-full"></div>
                  </div>
               </div>
               <div className="text-xs font-mono text-white/80 mx-2">02:14 / 05:00</div>
            </div>
            
            <div className="text-sm font-medium text-white truncate max-w-xs drop-shadow-md">
               A-Town Cinematic Engine Demo.mp4
            </div>
            
            <div className="flex items-center gap-4 text-white/80">
               <button className="hover:text-white transition-colors"><Settings className="w-5 h-5" /></button>
               <button className="hover:text-white transition-colors"><Maximize className="w-5 h-5" /></button>
            </div>
         </div>
      </div>
      
      {!isPlaying && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/20 backdrop-blur-sm pointer-events-none">
           <div className="w-20 h-20 bg-rose-500/80 rounded-full flex items-center justify-center shadow-[0_0_40px_rgba(244,63,94,0.3)] backdrop-blur text-white">
              <Play className="w-8 h-8 fill-current ml-1" />
           </div>
        </div>
      )}
    </div>
  );
}

export function ATCImageViewerView() {
  return (
     <div className="flex flex-col h-full bg-[#0f111a] text-slate-200">
        <div className="flex items-center justify-between p-3 bg-black/40 border-b border-white/5">
           <div className="text-sm font-medium text-white truncate px-2">IMG_9214_Cyberpunk.jpg</div>
           <div className="flex items-center gap-2">
              <button className="p-1.5 hover:bg-white/10 rounded text-slate-400 hover:text-white transition-colors"><Heart className="w-4 h-4" /></button>
              <button className="p-1.5 hover:bg-white/10 rounded text-slate-400 hover:text-white transition-colors"><RefreshCw className="w-4 h-4" /></button>
              <div className="w-px h-4 bg-white/10 mx-1"></div>
              <button className="p-1.5 hover:bg-white/10 rounded text-slate-400 hover:text-white transition-colors"><Minus className="w-4 h-4" /></button>
              <span className="text-xs font-mono px-2 text-white">100%</span>
              <button className="p-1.5 hover:bg-white/10 rounded text-slate-400 hover:text-white transition-colors"><Plus className="w-4 h-4" /></button>
           </div>
        </div>
        
        <div className="flex-1 overflow-hidden relative flex items-center justify-center p-8 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCI+CiAgPHJlY3Qgd2lkdGg9IjEwIiBoZWlnaHQ9IjEwIiBmaWxsPSIjMWExYTFhIi8+CiAgPHJlY3QgeD0iMTAiIHk9IjEwIiB3aWR0aD0iMTAiIGhlaWdodD0iMTAiIGZpbGw9IiMxYTFhMWEiLz4KICA8cmVjdCB4PSIxMCIgd2lkdGg9IjEwIiBoZWlnaHQ9IjEwIiBmaWxsPSIjMjIyMiIvPgogIDxyZWN0IHk9IjEwIiB3aWR0aD0iMTAiIGhlaWdodD0iMTAiIGZpbGw9IiMyMjIyIi8+Cjwvc3ZnPg==')]">
           <img 
              src="https://images.unsplash.com/photo-1542038784456-1ea8e935640e?w=1200&q=80" 
              alt="Cyberpunk view" 
              className="max-w-full max-h-full object-contain shadow-2xl ring-1 ring-white/10 rounded-sm"
           />
        </div>
        
        <div className="h-20 bg-black/40 border-t border-white/5 flex items-center px-4 gap-2 overflow-x-auto no-scrollbar">
           {[...Array(8)].map((_, i) => (
             <button key={i} className={`w-24 h-14 shrink-0 rounded border overflow-hidden ${i === 2 ? 'border-cyan-500 opacity-100 ring-2 ring-cyan-500/30' : 'border-transparent opacity-50 hover:opacity-80'}`}>
                <img src={`https://images.unsplash.com/photo-1542038784456-1ea8e935640e?w=200&q=80&auto=format&fit=crop&sig=${i}`} alt="thumbnail" className="w-full h-full object-cover" />
             </button>
           ))}
        </div>
     </div>
  );
}
