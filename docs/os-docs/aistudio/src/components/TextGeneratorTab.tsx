import React, { useState, useRef } from 'react';
import { Loader2, StopCircle, Type } from 'lucide-react';
import Markdown from 'react-markdown';

interface TextGeneratorTabProps {
  title: string;
  description: string;
  promptPlaceholder: string;
  icon: React.ReactNode;
  additionalContextPrompt?: string;
  buttonLabel?: string;
  onAssetGenerated?: (tag: string, name: string, desc: string) => void;
  assetTag?: string;
}

export function TextGeneratorTab({ title, description, promptPlaceholder, icon, additionalContextPrompt = "", buttonLabel = "Generieren", onAssetGenerated, assetTag = "Text" }: TextGeneratorTabProps) {
  const [topic, setTopic] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedText, setGeneratedText] = useState('');
  const [error, setError] = useState<string | null>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  const handleGenerate = async () => {
    if (!topic.trim()) {
      setError("Bitte fülle das Feld aus.");
      return;
    }
    setError(null);
    setIsGenerating(true);
    setGeneratedText('');

    abortControllerRef.current = new AbortController();

    try {
      const fullPrompt = `${additionalContextPrompt}\n\nUser Input:\n${topic}`;
      const response = await fetch('/api/franchise-factory/generate-text', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: fullPrompt }),
        signal: abortControllerRef.current.signal
      });

      if (!response.ok) throw new Error('Fehler beim Starten der Generierung');

      if (response.body) {
        const reader = response.body.getReader();
        const decoder = new TextDecoder('utf-8');
        let done = false;
        let fullTextContent = '';

        while (!done) {
          const { value, done: readerDone } = await reader.read();
          done = readerDone;

          if (value) {
            const chunkText = decoder.decode(value, { stream: true });
            const lines = chunkText.split('\n');
            for (const line of lines) {
              if (line.startsWith('data: ')) {
                const dataStr = line.substring(6);
                if (dataStr === '[DONE]') {
                  done = true;
                  break;
                }
                try {
                  const data = JSON.parse(dataStr);
                  if (data.error) {
                    setError(data.error);
                    done = true;
                    break;
                  }
                  if (data.text) {
                    fullTextContent += data.text;
                    setGeneratedText((prev) => prev + data.text);
                  }
                } catch (e) {
                  // Ignore
                }
              }
            }
          }
        }
        if (fullTextContent && onAssetGenerated) {
          onAssetGenerated(assetTag, `${title} - ${topic.substring(0, 30)}...`, fullTextContent);
        }
      }
    } catch (err: any) {
      if (err.name !== 'AbortError') setError(err.message || "Unbekannter Fehler.");
    } finally {
      setIsGenerating(false);
      abortControllerRef.current = null;
    }
  };

  const handleStop = () => {
    if (abortControllerRef.current) abortControllerRef.current.abort();
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
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Input / Thema</label>
            <textarea
              value={topic}
              onChange={(e) => setTopic(e.target.value)}
              placeholder={promptPlaceholder}
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50 min-h-[100px] resize-y"
              disabled={isGenerating}
            />
          </div>

          {error && (
            <div className="p-3 bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg">
              {error}
            </div>
          )}

          <div className="pt-2 flex gap-3">
            <button
              onClick={handleGenerate}
              disabled={isGenerating || !topic.trim()}
              className="flex-1 px-4 py-3 bg-cyan-600 hover:bg-cyan-500 text-white rounded-xl font-medium tracking-wide transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isGenerating ? (
                <><Loader2 className="w-4 h-4 animate-spin" /> Generiere...</>
              ) : (
                <>{icon} {buttonLabel}</>
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

      <div className="p-6 rounded-2xl bg-[#0b0f19] border border-white/5 flex flex-col max-h-[800px]">
        <h3 className="text-sm border-b border-white/5 pb-4 mb-4 font-mono text-cyan-400/80 uppercase tracking-wider">
          Output Preview
        </h3>
        <div className="flex-1 overflow-y-auto custom-scrollbar pr-2 pb-2">
          {generatedText ? (
            <div className="markdown-body prose prose-invert prose-cyan max-w-none prose-sm sm:prose-base">
              <Markdown>{generatedText}</Markdown>
            </div>
          ) : (
            <div className="h-full flex flex-col items-center justify-center text-slate-600 font-light gap-3 text-center">
              <div className="opacity-20">{icon}</div>
              <p>Noch kein Output generiert.<br/>Fülle die Details links aus und starte die KI.</p>
            </div>
          )}
          {isGenerating && (
            <div className="flex items-center gap-2 text-cyan-500 mt-4 text-sm animate-pulse">
              <Loader2 className="w-4 h-4 animate-spin" />
              <em>Die KI arbeitet...</em>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
