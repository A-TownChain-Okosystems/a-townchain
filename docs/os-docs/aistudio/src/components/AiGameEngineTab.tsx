import React, { useState, useRef } from 'react';
import { Gamepad2, Loader2, StopCircle, Code, Play } from 'lucide-react';
interface AiGameEngineTabProps { selectedFranchise: any; onAssetGenerated?: (tag: string, name: string, desc: string) => void; }
export function AiGameEngineTab({ selectedFranchise, onAssetGenerated }: AiGameEngineTabProps) {
  const [topic, setTopic] = useState('');
  const [flavor, setFlavor] = useState('2D Arcade / Retro Pixel');
  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedHtml, setGeneratedHtml] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<'preview' | 'code'>('preview');

  const abortControllerRef = useRef<AbortController | null>(null);

  const handleGenerateGame = async () => {
    if (!topic.trim()) {
      setError("Bitte gib ein Spielthema ein.");
      return;
    }
    setError(null);
    setIsGenerating(true);
    setGeneratedHtml('');
    setViewMode('preview');

    abortControllerRef.current = new AbortController();

    try {
      const franchiseContext = selectedFranchise ? `Franchise: ${selectedFranchise.name}. Beschreibung: ${selectedFranchise.description}. Lore Assets: ${JSON.stringify(selectedFranchise.assets)}` : '';
      const response = await fetch('/api/franchise-factory/generate-html-game', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ topic, flavor, franchiseContext }),
        signal: abortControllerRef.current.signal
      });

      if (!response.ok) throw new Error('Fehler beim Starten der Generierung');

      if (response.body) {
        const reader = response.body.getReader();
        const decoder = new TextDecoder('utf-8');
        let done = false;
        let htmlBuffer = '';

        while (!done) {
          const { value, done: readerDone } = await reader.read();
          done = readerDone;
          if (value) {
            const chunkText = decoder.decode(value, { stream: true });
            const lines = chunkText.split('\n');
            for (const line of lines) {
              if (line.startsWith('data: ') && line !== 'data: [DONE]') {
                try {
                  const data = JSON.parse(line.slice(6));
                  // Fix potential markdown code blocks stripping
                  let newText = data.text;
                  htmlBuffer += newText;
                  // Strip markdown wrapper if AI still gives it
                  let cleanHtml = htmlBuffer;
                  cleanHtml = cleanHtml.replace(/^```html\n?/, '').replace(/```$/, '');
                  setGeneratedHtml(cleanHtml);
                } catch (e) {
                  // ignore parse error
                }
              }
            }
          }
        }
        if (htmlBuffer && onAssetGenerated) {
          onAssetGenerated('Game', `Game: ${topic.substring(0, 30)}...`, htmlBuffer.replace(/^```html\n?/, '').replace(/```$/, ''));
        }
      }
    } catch (err: any) {
      if (err.name !== 'AbortError') {
        setError(err.message || 'Generierung fehlgeschlagen.');
      }
    } finally {
      setIsGenerating(false);
      abortControllerRef.current = null;
    }
  };

  const handleStop = () => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 h-[calc(100vh-250px)] min-h-[700px]">
      <div className="lg:col-span-4 p-6 rounded-2xl bg-[#0b0f19] border border-fuchsia-500/20 shadow-[0_0_20px_rgba(217,70,239,0.05)] flex flex-col gap-6 overflow-y-auto custom-scrollbar">
        <div className="flex items-center gap-3 border-b border-white/10 pb-4">
          <Gamepad2 className="w-5 h-5 text-fuchsia-400" />
          <h3 className="text-lg font-semibold text-white">KI Game Engine</h3>
        </div>

        <div className="flex flex-col gap-4">
          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Spielidee / Gameplay Target</label>
            <textarea
              value={topic}
              onChange={(e) => setTopic(e.target.value)}
              placeholder="Z.b. Ein endloser Spaceshooter mit Neon-Grafiken. Steuerung über Maus."
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-fuchsia-500/50 min-h-[140px] resize-y"
              disabled={isGenerating}
            />
          </div>

          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Art Style / Flavor</label>
            <select
              value={flavor}
              onChange={(e) => setFlavor(e.target.value)}
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-fuchsia-500/50"
              disabled={isGenerating}
            >
              <option value="2D Arcade / Retro Pixel">2D Arcade / Retro Pixel</option>
              <option value="Neon Cyberpunk">Neon Cyberpunk</option>
              <option value="Minimalist Geometry">Minimalist Geometry</option>
              <option value="Text Based Adventure">Text Based Adventure</option>
              <option value="Three.js 3D Wireframe">Three.js 3D Wireframe (experimentell)</option>
            </select>
          </div>

          {error && (
            <div className="p-3 bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg">
              {error}
            </div>
          )}

          <div className="pt-2 flex gap-3">
            <button
              onClick={handleGenerateGame}
              disabled={isGenerating || !topic.trim()}
              className="flex-1 px-4 py-3 bg-fuchsia-600 hover:bg-fuchsia-500 text-white rounded-xl font-medium tracking-wide transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isGenerating ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  Kompiliere...
                </>
              ) : (
                <>
                  <Play className="w-4 h-4" />
                  Build & Run
                </>
              )}
            </button>
            
            {isGenerating && (
              <button
                onClick={handleStop}
                className="px-4 py-3 bg-red-500/10 hover:bg-red-500/20 text-red-500 border border-red-500/30 rounded-xl transition-colors flex items-center justify-center gap-2"
                title="Abbrechen"
              >
                <StopCircle className="w-5 h-5" />
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="lg:col-span-8 p-1 rounded-2xl bg-black border border-white/10 flex flex-col relative overflow-hidden group">
        <div className="absolute top-4 right-4 z-10 flex border border-white/20 bg-black/80 rounded-lg shadow-lg overflow-hidden backdrop-blur-md opacity-0 group-hover:opacity-100 transition-opacity">
           <button onClick={() => setViewMode('preview')} className={`px-4 py-2 text-xs font-mono font-bold tracking-wider uppercase transition-colors flex items-center gap-2 ${viewMode === 'preview' ? 'bg-fuchsia-500/20 text-fuchsia-400' : 'text-slate-400 hover:text-white'}`}><Play className="w-3 h-3" /> Preview</button>
           <button onClick={() => setViewMode('code')} className={`px-4 py-2 text-xs font-mono font-bold tracking-wider uppercase transition-colors flex items-center gap-2 border-l border-white/20 ${viewMode === 'code' ? 'bg-fuchsia-500/20 text-fuchsia-400' : 'text-slate-400 hover:text-white'}`}><Code className="w-3 h-3" /> Source</button>
        </div>

        {viewMode === 'preview' ? (
          <div className="flex-1 w-full h-full bg-[#0b0f19] rounded-xl overflow-hidden relative">
            {generatedHtml ? (
              <iframe
                title="AI Game Preview"
                srcDoc={generatedHtml}
                className="w-full h-full border-none"
                sandbox="allow-scripts allow-same-origin"
              />
            ) : (
              <div className="absolute inset-0 flex flex-col items-center justify-center text-slate-600 font-light gap-3 text-center">
                <Gamepad2 className="w-16 h-16 opacity-10 mb-4" />
                <p>Der Game Canvas ist leer.</p>
                <p className="text-sm">Beschreibe deine Vision und lass den AI Engine Compiler das HTML5 Spiel generieren.</p>
              </div>
            )}
            {isGenerating && (
              <div className="absolute inset-0 bg-black/60 backdrop-blur-sm flex flex-col items-center justify-center text-fuchsia-400 font-mono text-sm z-20">
                <Loader2 className="w-8 h-8 animate-spin mb-4" />
                <div>Generiere Engine Code...</div>
              </div>
            )}
          </div>
        ) : (
           <div className="flex-1 w-full h-full bg-[#0b0f19] rounded-xl overflow-y-auto custom-scrollbar p-4">
              <pre className="text-xs text-emerald-300 font-mono whitespace-pre-wrap">
                 {generatedHtml || '// No code generated yet'}
              </pre>
           </div>
        )}
      </div>
    </div>
  );
}
