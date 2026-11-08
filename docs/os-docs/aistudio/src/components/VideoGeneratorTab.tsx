import React, { useState, useRef, useEffect } from 'react';
import { Loader2, VideoIcon } from 'lucide-react';

interface VideoGeneratorTabProps {
  title: string;
  description: string;
  promptPlaceholder: string;
  icon: React.ReactNode;
  additionalContextPrompt?: string;
  buttonLabel?: string;
  onAssetGenerated?: (tag: string, name: string, desc: string) => void;
  assetTag?: string;
}

export function VideoGeneratorTab({ title, description, promptPlaceholder, icon, additionalContextPrompt = "", buttonLabel = "Video generieren", onAssetGenerated, assetTag = "Video" }: VideoGeneratorTabProps) {
  const [videoPrompt, setVideoPrompt] = useState('');
  const [videoAspectRatio, setVideoAspectRatio] = useState('16:9');
  const [isGeneratingVideo, setIsGeneratingVideo] = useState(false);
  const [videoOperationName, setVideoOperationName] = useState<string | null>(null);
  const [videoPollingStatus, setVideoPollingStatus] = useState<string | null>(null);
  const [generatedVideoUrl, setGeneratedVideoUrl] = useState('');
  const [videoError, setVideoError] = useState<string | null>(null);
  const videoPollIntervalRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    return () => {
      if (videoPollIntervalRef.current) {
        clearInterval(videoPollIntervalRef.current);
      }
    };
  }, []);

  const handleGenerateVideo = async () => {
    if (!videoPrompt.trim()) {
      setVideoError("Bitte gib einen Prompt ein.");
      return;
    }
    setVideoError(null);
    setIsGeneratingVideo(true);
    setGeneratedVideoUrl('');
    setVideoOperationName(null);
    setVideoPollingStatus('Starte Video-Generierung...');

    try {
      const fullPrompt = `${additionalContextPrompt} ${videoPrompt}`;
      const response = await fetch('/api/franchise-factory/generate-video', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: fullPrompt, aspectRatio: videoAspectRatio }),
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.error || 'Fehler beim Starten der Generierung');
      
      const opName = data.operationName;
      setVideoOperationName(opName);
      setVideoPollingStatus('Generiere... (Das kann einige Minuten dauern)');

      if (videoPollIntervalRef.current) clearInterval(videoPollIntervalRef.current);
      videoPollIntervalRef.current = setInterval(async () => {
        try {
          const statusRes = await fetch('/api/franchise-factory/video-status', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ operationName: opName }),
          });
          const statusData = await statusRes.json();
          if (statusData.done) {
            if (videoPollIntervalRef.current) clearInterval(videoPollIntervalRef.current);
            setVideoPollingStatus('Video generiert, lade Video herunter...');
            await downloadVideo(opName);
          }
        } catch (pollErr) {
          console.error("Polling error", pollErr);
        }
      }, 10000);

    } catch (err: any) {
      setVideoError(err.message || "Ein unbekannter Fehler ist aufgetreten.");
      setIsGeneratingVideo(false);
      setVideoPollingStatus(null);
    }
  };

  const downloadVideo = async (opName: string) => {
    try {
      const downloadRes = await fetch('/api/franchise-factory/video-download', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ operationName: opName }),
      });
      if (!downloadRes.ok) {
        const errorData = await downloadRes.json();
        throw new Error(errorData.error || "Fehler beim Herunterladen des Videos");
      }
      const blob = await downloadRes.blob();
      const url = URL.createObjectURL(blob);
      setGeneratedVideoUrl(url);
      if (onAssetGenerated) {
        onAssetGenerated(assetTag, `${title} - ${videoPrompt.substring(0, 30)}...`, `Video (Local URL): ${url}`);
      }
    } catch (err: any) {
      setVideoError(err.message);
    } finally {
      setIsGeneratingVideo(false);
      setVideoPollingStatus(null);
    }
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div className="p-6 rounded-2xl bg-[#0b0f19] border border-cyan-500/20 shadow-[0_0_20px_rgba(6,182,212,0.05)] flex flex-col gap-6">
        <div className="flex items-center gap-3 border-b border-white/10 pb-4">
          <div className="text-cyan-400">{icon}</div>
          <h3 className="text-lg font-semibold text-white">{title}</h3>
        </div>
        <p className="text-slate-400 text-sm max-w-sm">{description}</p>
        
        <div className="flex flex-col gap-4">
          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Prompt</label>
            <textarea
              value={videoPrompt}
              onChange={(e) => setVideoPrompt(e.target.value)}
              placeholder={promptPlaceholder}
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50 min-h-[100px] resize-y"
              disabled={isGeneratingVideo}
            />
          </div>
          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Aspect Ratio (Format)</label>
            <select
              value={videoAspectRatio}
              onChange={(e) => setVideoAspectRatio(e.target.value)}
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50"
              disabled={isGeneratingVideo}
            >
              <option value="16:9">16:9 (Landscape)</option>
              <option value="9:16">9:16 (Portrait)</option>
            </select>
          </div>
          {videoError && (
            <div className="p-3 bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg">{videoError}</div>
          )}
          {videoPollingStatus && (
             <div className="p-3 bg-cyan-900/40 border border-cyan-500/30 text-cyan-300 text-sm rounded-lg flex items-center gap-2">
               <Loader2 className="w-4 h-4 animate-spin" /> {videoPollingStatus}
             </div>
          )}
          <button
            onClick={handleGenerateVideo}
            disabled={isGeneratingVideo || !videoPrompt.trim()}
            className="mt-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500 text-white rounded-xl font-medium tracking-wide transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            {isGeneratingVideo ? <><Loader2 className="w-4 h-4 animate-spin" /> Verarbeiten...</> : <>{icon} {buttonLabel}</>}
          </button>
        </div>
      </div>

      <div className="p-6 rounded-2xl bg-[#0b0f19] border border-white/5 flex flex-col items-center justify-center min-h-[400px]">
        {isGeneratingVideo && !generatedVideoUrl ? (
           <div className="flex flex-col items-center gap-4 text-cyan-400/80">
              <Loader2 className="w-8 h-8 animate-spin" />
              <span className="animate-pulse">{videoPollingStatus || "Video wird erstellt..."}</span>
           </div>
        ) : generatedVideoUrl ? (
          <video src={generatedVideoUrl} controls autoPlay loop className="max-w-full rounded-xl shadow-lg border border-white/10 bg-black/50" />
        ) : (
          <div className="text-center text-slate-600 font-light flex flex-col items-center gap-3">
             <div className="opacity-20">{icon}</div>
             <p>Output Preview<br/>Video wird hier angezeigt.</p>
          </div>
        )}
      </div>
    </div>
  );
}
