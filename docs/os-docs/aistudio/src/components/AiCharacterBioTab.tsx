import React, { useState, useRef } from 'react';
import { UserCircle, Loader2, StopCircle, Code, Play } from 'lucide-react';

interface AiCharacterBioTabProps { selectedFranchise: any; onAssetGenerated?: (tag: string, name: string, desc: string) => void; }
export function AiCharacterBioTab({ selectedFranchise, onAssetGenerated }: AiCharacterBioTabProps) {
  const [topic, setTopic] = useState('');
  const [flavor, setFlavor] = useState('RPG Character Sheet (Dark Mode)');
  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedHtml, setGeneratedHtml] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<'preview' | 'code'>('preview');

  const abortControllerRef = useRef<AbortController | null>(null);

  const handleGenerateCharacter = async () => {
    if (!topic.trim()) {
      setError("Bitte beschreibe einen Charakter.");
      return;
    }
    setError(null);
    setIsGenerating(true);
    setGeneratedHtml('');
    setViewMode('preview');

    abortControllerRef.current = new AbortController();

    try {
      const franchiseContext = selectedFranchise ? `Franchise: ${selectedFranchise.name}. Beschreibung: ${selectedFranchise.description}. Lore Assets: ${JSON.stringify(selectedFranchise.assets)}` : '';
      const response = await fetch('/api/franchise-factory/generate-character', {
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
                  let newText = data.text;
                  htmlBuffer += newText;
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
          onAssetGenerated('Charakter', `Char: ${topic.substring(0, 30)}...`, htmlBuffer.replace(/^```html\n?/, '').replace(/```$/, ''));
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
      <div className="lg:col-span-4 p-6 rounded-2xl bg-[#0b0f19] border border-cyan-500/20 shadow-[0_0_20px_rgba(6,182,212,0.05)] flex flex-col gap-6 overflow-y-auto custom-scrollbar">
        <div className="flex items-center gap-3 border-b border-white/10 pb-4">
          <UserCircle className="w-5 h-5 text-cyan-400" />
          <h3 className="text-lg font-semibold text-white">Character Bio Engine</h3>
        </div>

        <div className="flex flex-col gap-4">
          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Charakter Beschreibung</label>
            <textarea
              value={topic}
              onChange={(e) => setTopic(e.target.value)}
              placeholder="Z.b. Ein desertierter Elite-Soldat, der nun als Untergrund-Doc in Neo-Berlin arbeitet..."
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50 min-h-[140px] resize-y"
              disabled={isGenerating}
            />
          </div>

          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Visual Style</label>
            <select
              value={flavor}
              onChange={(e) => setFlavor(e.target.value)}
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50"
              disabled={isGenerating}
            >
              <option value="RPG Character Sheet (Dark Mode)">RPG Character Sheet (Dark Mode)</option>
              <option value="Cyberpunk Dossier">Cyberpunk Dossier</option>
              <option value="Fantasy Scroll / Parchment">Fantasy Scroll</option>
              <option value="Minimalist Profile Card">Minimalist Profile Card</option>
              <option value="Top Secret Government File">Top Secret Government File</option>
            </select>
          </div>

          {error && (
            <div className="p-3 bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg">
              {error}
            </div>
          )}

          <div className="pt-2 flex gap-3">
            <button
              onClick={handleGenerateCharacter}
              disabled={isGenerating || !topic.trim()}
              className="flex-1 px-4 py-3 bg-cyan-600 hover:bg-cyan-500 text-white rounded-xl font-medium tracking-wide transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isGenerating ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  Schreibe Code...
                </>
              ) : (
                <>
                  <UserCircle className="w-4 h-4" />
                  Bio generieren
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
           <button onClick={() => setViewMode('preview')} className={`px-4 py-2 text-xs font-mono font-bold tracking-wider uppercase transition-colors flex items-center gap-2 ${viewMode === 'preview' ? 'bg-cyan-500/20 text-cyan-400' : 'text-slate-400 hover:text-white'}`}><Play className="w-3 h-3" /> Preview</button>
           <button onClick={() => setViewMode('code')} className={`px-4 py-2 text-xs font-mono font-bold tracking-wider uppercase transition-colors flex items-center gap-2 border-l border-white/20 ${viewMode === 'code' ? 'bg-cyan-500/20 text-cyan-400' : 'text-slate-400 hover:text-white'}`}><Code className="w-3 h-3" /> Source</button>
        </div>

        {viewMode === 'preview' ? (
          <div className="flex-1 w-full h-full bg-[#0b0f19] rounded-xl overflow-hidden relative">
            {generatedHtml ? (
              <iframe
                title="AI Character Bio Preview"
                srcDoc={generatedHtml}
                className="w-full h-full border-none"
                sandbox="allow-scripts allow-same-origin"
              />
            ) : (
              <div className="absolute inset-0 flex flex-col items-center justify-center text-slate-600 font-light gap-3 text-center">
                <UserCircle className="w-16 h-16 opacity-10 mb-4" />
                <p>Der Character Sheet Canvas ist leer.</p>
                <p className="text-sm">Beschreibe einen Charakter und generiere ein interaktives Profil.</p>
              </div>
            )}
            {isGenerating && (
              <div className="absolute inset-0 bg-black/60 backdrop-blur-sm flex flex-col items-center justify-center text-cyan-400 font-mono text-sm z-20">
                <Loader2 className="w-8 h-8 animate-spin mb-4" />
                <div>Generiere Character Code...</div>
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
