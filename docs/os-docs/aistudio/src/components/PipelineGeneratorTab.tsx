import React, { useState, useRef } from 'react';
import { Loader2, Workflow, CheckCircle2, Circle } from 'lucide-react';
import Markdown from 'react-markdown';

interface PipelineGeneratorTabProps {
  title: string;
  description: string;
  onAssetGenerated?: (tag: string, name: string, desc: string) => void;
}

export function PipelineGeneratorTab({ title, description, onAssetGenerated }: PipelineGeneratorTabProps) {
  const [topic, setTopic] = useState('');
  const [isRunning, setIsRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // States for different steps
  const [currentStep, setCurrentStep] = useState<number>(0);
  const [loreOutput, setLoreOutput] = useState('');
  const [charsOutput, setCharsOutput] = useState('');
  const [imgOutputUrl, setImgOutputUrl] = useState('');
  const [threeOutput, setThreeOutput] = useState('');
  const [gameOutput, setGameOutput] = useState('');
  const [animOutput, setAnimOutput] = useState('');
  const [audioOutput, setAudioOutput] = useState('');
  const [timelineOutput, setTimelineOutput] = useState('');
  const [sfxOutput, setSfxOutput] = useState('');
  const [videoOutput, setVideoOutput] = useState('');
  const [vfxOutput, setVfxOutput] = useState('');
  const [cinematicOutput, setCinematicOutput] = useState('');
  const [syncOutput, setSyncOutput] = useState('');
  const [consistencyOutput, setConsistencyOutput] = useState('');

  const abortControllerRef = useRef<AbortController | null>(null);

  const steps = [
    "Initialisierung...",
    "Generiere Core Lore & Worldbuilding...",
    "Erstelle Hauptcharaktere...",
    "Generiere Key Visual (Concept Art)...",
    "Generiere Timeline...",
    "Generiere 3D Render Szene...",
    "Generiere Playable Game Prototyp...",
    "Generiere Animation...",
    "Generiere Musik & Audio Track...",
    "Generiere Sound Effekte...",
    "Generiere Video Pipeline & Drehbuch...",
    "Generiere Video Effekte (VFX) Plan...",
    "Generiere Cinematic Pitch...",
    "Generiere Dialog Synchronisation...",
    "Führe Universum Konsistenz-Prüfung durch...",
    "Pipeline abgeschlossen."
  ];

  const streamFromApi = async (url: string, body: any, setOutputText: React.Dispatch<React.SetStateAction<string>>, signal: AbortSignal, isHtml: boolean = false) => {
    const res = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
      signal
    });
    
    if (!res.ok) throw new Error(`Fehler bei ${url}`);
    
    if (res.body) {
      const reader = res.body.getReader();
      const decoder = new TextDecoder('utf-8');
      let done = false;
      let buffer = '';
      while (!done) {
        const { value, done: readerDone } = await reader.read();
        done = readerDone;
        if (value) {
          const chunkText = decoder.decode(value, { stream: true });
          const lines = chunkText.split('\n');
          for (const line of lines) {
            if (line.startsWith('data: ')) {
              const dataStr = line.substring(6);
              if (dataStr === '[DONE]') { done = true; break; }
              try {
                const data = JSON.parse(dataStr);
                if (data.error) throw new Error(data.error);
                if (data.text) {
                  buffer += data.text;
                  if (isHtml) {
                    setOutputText(buffer.replace(/^```html\n?/, '').replace(/```$/, ''));
                  } else {
                    setOutputText(buffer);
                  }
                }
              } catch (e) {}
            }
          }
        }
      }
      return isHtml ? buffer.replace(/^```html\n?/, '').replace(/```$/, '') : buffer;
    }
    return '';
  };

  const handleRunPipeline = async () => {
    if (!topic.trim()) {
      setError("Bitte gib eine Kernidee ein.");
      return;
    }

    setError(null);
    setIsRunning(true);
    setCurrentStep(1);
    setLoreOutput('');
    setCharsOutput('');
    setImgOutputUrl('');
    setThreeOutput('');
    setGameOutput('');
    setAnimOutput('');
    setAudioOutput('');
    setTimelineOutput('');
    setSfxOutput('');
    setVideoOutput('');
    setVfxOutput('');
    setCinematicOutput('');
    setSyncOutput('');
    setConsistencyOutput('');

    abortControllerRef.current = new AbortController();
    const signal = abortControllerRef.current.signal;

    try {
      // Step 1: Lore Generator
      const lorePrompt = `You are a worldbuilding AI. Based on the following seed idea, generate a detailed core lore and worldbuilding summary for a new franchise.\n\nSeed Idea: ${topic}`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: lorePrompt }, setLoreOutput, signal);

      setCurrentStep(2);

      // Step 2: Characters
      const charsPrompt = `Based on the following world lore, create 3 compelling main characters with detailed biographies, motivations, and visual descriptions.\n\nLore:\n${loreOutput || topic}`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: charsPrompt }, setCharsOutput, signal);

      setCurrentStep(3);

      // Step 3: Key Visual (Image)
      const imgPromptText = `A stunning cinematic key visual concept art for a franchise with the following main idea: ${topic}. Highly detailed, masterpiece.`;
      const imgRes = await fetch('/api/franchise-factory/generate-image', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: imgPromptText, aspectRatio: '16:9' }),
        signal: signal
      });

      const imgData = await imgRes.json();
      if (!imgRes.ok) throw new Error(imgData.error || 'Fehler bei der Bildgenerierung.');
      
      if (imgData.imageUrl) {
        setImgOutputUrl(imgData.imageUrl);
      }

      setCurrentStep(4);

      // Step 4: Timeline
      await streamFromApi('/api/franchise-factory/generate-timeline', { topic: `Key historical events of: ${topic}`, flavor: 'Vertical Interactive UI', franchiseContext: '' }, setTimelineOutput, signal, true);

      setCurrentStep(5);
      
      // Step 5: 3D Render
      await streamFromApi('/api/franchise-factory/generate-3d', { topic: `A symbolic 3D object for: ${topic}`, flavor: 'Cyberpunk / Neon', franchiseContext: '' }, setThreeOutput, signal, true);

      setCurrentStep(6);
      
      // Step 6: Game
      await streamFromApi('/api/franchise-factory/generate-html-game', { topic: `A minigame showcasing: ${topic}`, flavor: '2D Arcade / Retro Pixel', franchiseContext: '' }, setGameOutput, signal, true);

      setCurrentStep(7);
      
      // Step 7: Animation
      await streamFromApi('/api/franchise-factory/generate-animation', { topic: `An ambient animated backdrop for: ${topic}`, flavor: 'HTML5 Canvas', franchiseContext: '' }, setAnimOutput, signal, true);

      setCurrentStep(8);
      
      // Step 8: Audio Track
      await streamFromApi('/api/franchise-factory/generate-audio', { topic: `An atmospheric background track for: ${topic}`, flavor: 'Web Audio API Synth', franchiseContext: '' }, setAudioOutput, signal, true);

      setCurrentStep(9);
      
      // Step 9: Sound Effects
      const sfxPrompt = `Based on the franchise: ${topic}, conceptualize 5 key sound effects (SFX) that define this universe. Describe their texture, how they are generated, and where they apply.`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: sfxPrompt }, setSfxOutput, signal);

      setCurrentStep(10);
      
      // Step 10: Video Pipeline
      const videoPrompt = `Write a short pitch and technical pipeline overview for a pilot episode or promotional video for the franchise concept: ${topic}.`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: videoPrompt }, setVideoOutput, signal);

      setCurrentStep(11);
      
      // Step 11: VFX
      const vfxPrompt = `Outline the primary Video Effects (VFX) strategies and particle systems required for the world of: ${topic}.`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: vfxPrompt }, setVfxOutput, signal);

      setCurrentStep(12);

      // Step 12: Cinematic
      const cinematicPrompt = `Write a script for a 60-second cinematic intro cutscene for the game/movie of: ${topic}. Include camera angles and lighting cues.`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: cinematicPrompt }, setCinematicOutput, signal);

      setCurrentStep(13);

      // Step 13: Sync
      const syncPrompt = `Generate a dialogue script snippet showcasing character voice characteristics and lip-synchronization focus points for the cast of: ${topic}.`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: syncPrompt }, setSyncOutput, signal);

      setCurrentStep(14);

      // Step 14: Consistency
      const consistencyPrompt = `Act as the franchise loremaster and QA lead. Perform a consistency check on the premise: ${topic}. Identify potential worldbuilding plot-holes and how to solve them.`;
      await streamFromApi('/api/franchise-factory/generate-text', { prompt: consistencyPrompt }, setConsistencyOutput, signal);

      setCurrentStep(15);

    } catch (err: any) {
      if (err.name !== 'AbortError') setError(err.message || 'Pipeline Fehler');
      setIsRunning(false);
    } finally {
      setIsRunning(false);
      abortControllerRef.current = null;
    }
  };

  const handleStop = () => {
    if (abortControllerRef.current) abortControllerRef.current.abort();
    setIsRunning(false);
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div className="p-6 rounded-2xl bg-[#0b0f19] border border-purple-500/20 shadow-[0_0_20px_rgba(168,85,247,0.05)] flex flex-col gap-6">
        <div className="flex items-center gap-3 border-b border-white/10 pb-4">
          <Workflow className="w-5 h-5 text-purple-400" />
          <h3 className="text-lg font-semibold text-white">{title}</h3>
        </div>
        <p className="text-slate-400 text-sm max-w-sm">{description}</p>
        
        <div className="flex flex-col gap-4">
          <div>
            <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Kernidee / Pitch</label>
            <textarea
              value={topic}
              onChange={(e) => setTopic(e.target.value)}
              placeholder="Eine Fraktion von Rebellen..."
              className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-purple-500/50 min-h-[100px] resize-y"
              disabled={isRunning}
            />
          </div>

          {error && (
            <div className="p-3 bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg">{error}</div>
          )}

          <div className="pt-2 flex gap-3">
            <button
              onClick={handleRunPipeline}
              disabled={isRunning || !topic.trim()}
              className="flex-1 px-4 py-3 bg-purple-600 hover:bg-purple-500 text-white rounded-xl font-medium tracking-wide transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isRunning ? <><Loader2 className="w-4 h-4 animate-spin" /> Pipeline läuft...</> : <><Workflow className="w-4 h-4" /> Vollständige Pipeline Starten</>}
            </button>
            
            {isRunning && (
              <button
                onClick={handleStop}
                className="px-4 py-3 bg-red-500/10 hover:bg-red-500/20 text-red-500 border border-red-500/30 rounded-xl transition-colors flex items-center justify-center gap-2"
                title="Abbrechen"
              >
                Abbrechen
              </button>
            )}
          </div>
        </div>

        <div className="mt-4 border-t border-white/10 pt-4">
          <h4 className="text-xs font-mono uppercase tracking-wider text-slate-400 mb-3">Pipeline Status</h4>
          <div className="flex flex-col gap-3">
            {steps.map((stepText, idx) => {
              const StepIcon = currentStep > idx ? CheckCircle2 : (currentStep === idx ? Loader2 : Circle);
              const colorClass = currentStep > idx ? 'text-green-400' : (currentStep === idx ? 'text-purple-400' : 'text-slate-600');
              const textClass = currentStep >= idx ? 'text-white' : 'text-slate-500';
              
              if (idx === 0) return null;

              return (
                <div key={idx} className={`flex items-center gap-3 text-sm ${textClass}`}>
                  <StepIcon className={`w-4 h-4 ${colorClass} ${currentStep === idx ? 'animate-spin' : ''}`} />
                  {stepText}
                </div>
              );
            })}
          </div>
        </div>

      </div>

      <div className="p-6 rounded-2xl bg-[#0b0f19] border border-white/5 flex flex-col h-[800px]">
        <h3 className="text-sm border-b border-white/5 pb-4 mb-4 font-mono text-purple-400/80 uppercase tracking-wider">
          Generierte Assets
        </h3>
        
        <div className="flex-1 overflow-y-auto custom-scrollbar pr-2 pb-2 flex flex-col gap-8">
          {!loreOutput && !charsOutput && !imgOutputUrl && !timelineOutput && !threeOutput && !gameOutput && !animOutput && !audioOutput && !sfxOutput && !videoOutput && !vfxOutput && !cinematicOutput && !syncOutput && !consistencyOutput && !isRunning && (
            <div className="h-full flex flex-col items-center justify-center text-slate-600 font-light gap-3 text-center">
              <Workflow className="w-10 h-10 opacity-20" />
              <p>Noch keine Pipeline gestartet.<br/>Ergebnisse erscheinen hier Schritt für Schritt.</p>
            </div>
          )}

          {loreOutput && (
            <div>
              <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Core Lore</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{loreOutput}</Markdown>
              </div>
            </div>
          )}

          {charsOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Charaktere</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{charsOutput}</Markdown>
              </div>
            </div>
          )}

          {imgOutputUrl && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Key Visual Concept</h4>
               <img src={imgOutputUrl} alt="Key Visual Concept" className="rounded-xl border border-white/10 w-full" referrerPolicy="no-referrer" />
            </div>
          )}

          {timelineOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Timeline</h4>
               <iframe srcDoc={timelineOutput} sandbox="allow-scripts allow-same-origin" className="w-full h-[400px] border-none rounded-xl" />
            </div>
          )}
          
          {threeOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> 3D Render Prototyp</h4>
               <iframe srcDoc={threeOutput} sandbox="allow-scripts allow-same-origin" className="w-full h-[400px] border-none rounded-xl" />
            </div>
          )}

          {gameOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Playable Game</h4>
               <iframe srcDoc={gameOutput} sandbox="allow-scripts allow-same-origin" className="w-full h-[400px] border-none rounded-xl" />
            </div>
          )}

          {animOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Frontend Animation</h4>
               <iframe srcDoc={animOutput} sandbox="allow-scripts allow-same-origin" className="w-full h-[400px] border-none rounded-xl" />
            </div>
          )}

          {audioOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Audio Track</h4>
               <iframe srcDoc={audioOutput} sandbox="allow-scripts allow-same-origin" className="w-full h-[200px] border-none rounded-xl" />
            </div>
          )}

          {sfxOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Sound Effekte (SFX)</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{sfxOutput}</Markdown>
              </div>
            </div>
          )}

          {videoOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Video & Drehbuch</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{videoOutput}</Markdown>
              </div>
            </div>
          )}

          {vfxOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Video Effekte (VFX)</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{vfxOutput}</Markdown>
              </div>
            </div>
          )}

          {cinematicOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Cinematic Pitch</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{cinematicOutput}</Markdown>
              </div>
            </div>
          )}

          {syncOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Dialog & Synchronization</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{syncOutput}</Markdown>
              </div>
            </div>
          )}

          {consistencyOutput && (
            <div>
               <h4 className="text-white font-semibold mb-2 flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-purple-500"></div> Konsistenz-Prüfung</h4>
              <div className="markdown-body prose prose-invert prose-purple max-w-none prose-sm">
                <Markdown>{consistencyOutput}</Markdown>
              </div>
            </div>
          )}

        </div>
      </div>
    </div>
  );
}

