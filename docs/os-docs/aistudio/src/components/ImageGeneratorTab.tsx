import React, { useState } from 'react';
import { Loader2, ImageIcon } from 'lucide-react';

interface ImageGeneratorTabProps {
  title: string;
  description: string;
  promptPlaceholder: string;
  icon: React.ReactNode;
  additionalContextPrompt?: string;
  buttonLabel?: string;
  onAssetGenerated?: (tag: string, name: string, desc: string) => void;
  assetTag?: string;
}

export function ImageGeneratorTab({ title, description, promptPlaceholder, icon, additionalContextPrompt = "", buttonLabel = "Bild generieren", onAssetGenerated, assetTag = "Image" }: ImageGeneratorTabProps) {
  const [imagePrompt, setImagePrompt] = useState('');
  const [imageAspectRatio, setImageAspectRatio] = useState('16:9');
  const [isGeneratingImage, setIsGeneratingImage] = useState(false);
  const [generatedImageUrl, setGeneratedImageUrl] = useState('');
  const [imageError, setImageError] = useState<string | null>(null);

  const handleGenerateImage = async () => {
    if (!imagePrompt.trim()) {
      setImageError("Bitte gib einen Prompt ein.");
      return;
    }
    setImageError(null);
    setIsGeneratingImage(true);
    setGeneratedImageUrl('');

    try {
      const fullPrompt = `${additionalContextPrompt} ${imagePrompt}`;
      const response = await fetch('/api/franchise-factory/generate-image', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: fullPrompt, aspectRatio: imageAspectRatio }),
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.error || 'Fehler beim Starten der Generierung');
      if (data.imageUrl) {
        setGeneratedImageUrl(data.imageUrl);
        if (onAssetGenerated) {
          onAssetGenerated(assetTag, `${title} - ${imagePrompt.substring(0, 30)}...`, `Image URL: ${data.imageUrl}`);
        }
      }
    } catch (err: any) {
      setImageError(err.message || "Ein unbekannter Fehler ist aufgetreten.");
    } finally {
      setIsGeneratingImage(false);
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
              value={imagePrompt}
              onChange={(e) => setImagePrompt(e.target.value)}
              placeholder={promptPlaceholder}
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50 min-h-[100px] resize-y"
              disabled={isGeneratingImage}
            />
          </div>
          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Aspect Ratio (Format)</label>
            <select
              value={imageAspectRatio}
              onChange={(e) => setImageAspectRatio(e.target.value)}
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50"
              disabled={isGeneratingImage}
            >
              <option value="16:9">16:9 (Landscape)</option>
              <option value="1:1">1:1 (Square)</option>
              <option value="9:16">9:16 (Portrait)</option>
              <option value="4:3">4:3 (Classic Thumbnail)</option>
            </select>
          </div>
          {imageError && (
            <div className="p-3 bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg">{imageError}</div>
          )}
          <button
            onClick={handleGenerateImage}
            disabled={isGeneratingImage || !imagePrompt.trim()}
            className="mt-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500 text-white rounded-xl font-medium tracking-wide transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            {isGeneratingImage ? <><Loader2 className="w-4 h-4 animate-spin" /> Generiere...</> : <>{icon} {buttonLabel}</>}
          </button>
        </div>
      </div>

      <div className="p-6 rounded-2xl bg-[#0b0f19] border border-white/5 flex flex-col items-center justify-center min-h-[400px]">
        {isGeneratingImage ? (
           <div className="flex flex-col items-center gap-4 text-cyan-400/80">
              <Loader2 className="w-8 h-8 animate-spin" />
              <span className="animate-pulse">KI generiert das Bild...</span>
           </div>
        ) : generatedImageUrl ? (
          <img src={generatedImageUrl} alt={imagePrompt} referrerPolicy="no-referrer" className="max-w-full rounded-xl shadow-lg border border-white/10" />
        ) : (
          <div className="text-center text-slate-600 font-light flex flex-col items-center gap-3">
             <div className="opacity-20">{icon}</div>
             <p>Output Preview<br/>Bild wird hier angezeigt.</p>
          </div>
        )}
      </div>
    </div>
  );
}
