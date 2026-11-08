import React, { useState } from "react";
import {
  Volume2,
  Play,
  Square,
  Settings2,
  FileAudio,
  Plus,
  Mic,
} from "lucide-react";

export function AudioEnginePanel() {
  const [tracks, setTracks] = useState([
    {
      id: "1",
      name: "Ambient Loop 1",
      type: "bgm",
      duration: "2:45",
      status: "ready",
    },
    {
      id: "2",
      name: "UI Click Soft",
      type: "sfx",
      duration: "0:01",
      status: "ready",
    },
    {
      id: "3",
      name: "Error Beep",
      type: "sfx",
      duration: "0:02",
      status: "ready",
    },
  ]);
  const [isPlaying, setIsPlaying] = useState<string | null>(null);

  const togglePlay = (id: string) => {
    if (isPlaying === id) {
      setIsPlaying(null);
    } else {
      setIsPlaying(id);
    }
  };

  return (
    <div className="flex-1 bg-[#101014] text-slate-200 outline-none flex flex-col font-sans h-full overflow-hidden">
      <div className="px-6 py-4 border-b border-white/5 flex items-center justify-between bg-black/20">
        <div className="flex items-center gap-3">
          <div className="bg-purple-500/20 p-2 rounded-lg">
            <Volume2 className="w-5 h-5 text-purple-400" />
          </div>
          <div>
            <h1 className="text-lg font-bold text-white tracking-tight">
              Audio Engine
            </h1>
            <p className="text-xs text-slate-400">
              Verwalte Sound-Assets und Audio-Logik.
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <button className="flex items-center gap-1.5 px-3 py-1.5 bg-white/5 hover:bg-white/10 text-slate-300 rounded text-xs font-bold transition-colors">
            <Mic className="w-3.5 h-3.5" />
            Aufnehmen
          </button>
          <button className="flex items-center gap-1.5 px-3 py-1.5 bg-purple-500/20 hover:bg-purple-500/30 text-purple-400 rounded text-xs font-bold transition-colors">
            <Plus className="w-3.5 h-3.5" />
            Importieren
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        <div>
          <h2 className="text-sm font-bold text-slate-300 mb-3 uppercase tracking-wider flex items-center gap-2">
            <Settings2 className="w-4 h-4 text-slate-500" />
            Audio Controller (Global)
          </h2>
          <div className="bg-black/40 border border-white/5 p-4 rounded-xl">
            <div className="space-y-4">
              <div>
                <div className="flex justify-between text-xs mb-1">
                  <span className="text-slate-400">Master Volume</span>
                  <span className="text-cyan-400 font-mono">85%</span>
                </div>
                <div className="h-1.5 bg-white/10 rounded-full overflow-hidden">
                  <div className="h-full bg-cyan-500 w-[85%] rounded-full shadow-[0_0_10px_rgba(6,182,212,0.5)]"></div>
                </div>
              </div>
              <div>
                <div className="flex justify-between text-xs mb-1">
                  <span className="text-slate-400">SFX Volume</span>
                  <span className="text-purple-400 font-mono">100%</span>
                </div>
                <div className="h-1.5 bg-white/10 rounded-full overflow-hidden">
                  <div className="h-full bg-purple-500 w-full rounded-full shadow-[0_0_10px_rgba(168,85,247,0.5)]"></div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div>
          <h2 className="text-sm font-bold text-slate-300 mb-3 uppercase tracking-wider flex items-center gap-2">
            <FileAudio className="w-4 h-4 text-slate-500" />
            Audio Asset Bibliothek
          </h2>
          <div className="grid gap-2">
            {tracks.map((track) => (
              <div
                key={track.id}
                className="flex items-center gap-4 bg-black/40 border border-white/5 p-3 rounded-lg hover:border-white/10 transition-colors group"
              >
                <button
                  onClick={() => togglePlay(track.id)}
                  className={`w-8 h-8 rounded-full flex items-center justify-center transition-colors ${isPlaying === track.id ? "bg-purple-500 text-white shadow-[0_0_15px_rgba(168,85,247,0.5)]" : "bg-white/5 text-slate-400 group-hover:bg-white/10 group-hover:text-white"}`}
                >
                  {isPlaying === track.id ? (
                    <Square className="w-4 h-4" />
                  ) : (
                    <Play className="w-4 h-4 ml-0.5" />
                  )}
                </button>
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between items-center mb-1">
                    <span className="text-sm font-medium text-slate-200 truncate">
                      {track.name}
                    </span>
                    <span className="text-[10px] font-mono text-slate-500">
                      {track.duration}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span
                      className={`text-[9px] px-1.5 py-0.5 rounded font-bold uppercase ${track.type === "bgm" ? "bg-emerald-500/20 text-emerald-400" : "bg-amber-500/20 text-amber-400"}`}
                    >
                      {track.type}
                    </span>
                    <span className="text-[10px] text-slate-500">
                      {track.status === "ready"
                        ? "Bereit zum Laden"
                        : "Wird geladen..."}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
