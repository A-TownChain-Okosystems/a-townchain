import React, { useState, useRef, useEffect } from 'react';
import { Factory, BookOpen, Loader2, StopCircle, ImageIcon, VideoIcon, Database, Gamepad2, FileText, UserCircle, Hexagon, Image as ImageIcons, Film, Workflow, Github, GitBranch, Globe, Wand2, Music, History, Volume2, MonitorPlay, Mic, CheckCircle, Compass, Activity, Users, Map, Terminal, CloudFog, Briefcase, Cog, Layout, Server, Code, Megaphone, Share2, Bot, TrendingUp, Milestone, ListTodo, List } from 'lucide-react';
import Markdown from 'react-markdown';
import { TextGeneratorTab } from './TextGeneratorTab';
import { ImageGeneratorTab } from './ImageGeneratorTab';
import { VideoGeneratorTab } from './VideoGeneratorTab';
import { PipelineGeneratorTab } from './PipelineGeneratorTab';
import { AiGameEngineTab } from './AiGameEngineTab';
import { Ai3DRenderEngineTab } from './Ai3DRenderEngineTab';
import { AiAnimationEngineTab } from './AiAnimationEngineTab';
import { AiAudioEngineTab } from './AiAudioEngineTab';
import { AiCharacterBioTab } from './AiCharacterBioTab';
import { AiTimelineEngineTab } from './AiTimelineEngineTab';

interface Asset {
  id: string;
  tag: string;
  name: string;
  desc: string;
}

interface Franchise {
  id: string;
  name: string;
  description: string;
  repository?: string;
  wikiUrl?: string;
  assets: Asset[];
}

interface Studio {
  id: string;
  name: string;
  mainRepository?: string;
  franchises: Franchise[];
}

export function FranchiseFactoryView() {
  const [activeTab, setActiveTab] = useState<'repositories' | 'database' | 'book' | 'image' | 'video' | 'game' | 'wiki' | 'lore' | 'char_bios' | 'npc' | 'world_design' | 'prompts' | 'ambiente' | 'animation' | 'game_assets' | 'textures' | 'mesh' | 'effect' | 'musik' | 'film' | 'serie' | 'timeline' | 'sfx' | 'cinematic' | 'sync' | 'av_sync' | 'consistency' | 'bewerbung_assets' | 'game_mechanics' | 'frontend' | 'backend' | 'all_in_one' | 'franchise_code' | 'avatar_ki' | 'werbung_ki' | 'sozialmedia_ki' | 'sozialmedia_avatar_ki' | 'marketing_ki' | 'roadmap_ki' | 'todo_ki' | 'inhaltsverzeichnis_ki'>('repositories');

  // Studio & Franchise Management
  const [studios, setStudios] = useState<Studio[]>([]);
  const [selectedStudioId, setSelectedStudioId] = useState<string>('');
  const [selectedFranchiseId, setSelectedFranchiseId] = useState<string>('');
  const [isCreatingStudio, setIsCreatingStudio] = useState(false);
  const [newStudioName, setNewStudioName] = useState('');
  const [isCreatingFranchise, setIsCreatingFranchise] = useState(false);
  const [newFranchiseName, setNewFranchiseName] = useState('');
  const [newFranchiseDesc, setNewFranchiseDesc] = useState('');

  const selectedStudio = studios.find(s => s.id === selectedStudioId) || studios[0];
  const selectedFranchise = selectedStudio?.franchises.find(f => f.id === selectedFranchiseId) || selectedStudio?.franchises[0];

  const handleAssetGenerated = (tag: string, name: string, desc: string) => {
    if (!selectedStudioId || !selectedFranchiseId) return;

    const newAsset: Asset = {
      id: `AST-${Date.now()}-${Math.floor(Math.random() * 1000)}`,
      tag,
      name,
      desc
    };

    setStudios(currentStudios => currentStudios.map(studio => {
      if (studio.id === selectedStudioId) {
        return {
          ...studio,
          franchises: studio.franchises.map(franchise => {
            if (franchise.id === selectedFranchiseId) {
              return {
                ...franchise,
                assets: [...franchise.assets, newAsset]
              };
            }
            return franchise;
          })
        };
      }
      return studio;
    }));
  };

  // Book generator state
  const [topic, setTopic] = useState('');
  const [genre, setGenre] = useState('Sci-Fi');
  const [chapterCount, setChapterCount] = useState(3);
  const [isGeneratingBook, setIsGeneratingBook] = useState(false);
  const [generatedBook, setGeneratedBook] = useState('');
  const [bookError, setBookError] = useState<string | null>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  // Image generator state
  const [imagePrompt, setImagePrompt] = useState('');
  const [imageAspectRatio, setImageAspectRatio] = useState('16:9');
  const [isGeneratingImage, setIsGeneratingImage] = useState(false);
  const [generatedImageUrl, setGeneratedImageUrl] = useState('');
  const [imageError, setImageError] = useState<string | null>(null);

  // Video generator state
  const [videoPrompt, setVideoPrompt] = useState('');
  const [videoAspectRatio, setVideoAspectRatio] = useState('16:9');
  const [videoFormat, setVideoFormat] = useState('short');
  const [isGeneratingVideo, setIsGeneratingVideo] = useState(false);
  const [videoOperationName, setVideoOperationName] = useState<string | null>(null);
  const [videoPollingStatus, setVideoPollingStatus] = useState<string | null>(null);
  const [generatedVideoUrl, setGeneratedVideoUrl] = useState('');
  const [videoError, setVideoError] = useState<string | null>(null);
  const videoPollIntervalRef = useRef<NodeJS.Timeout | null>(null);

  // Game generator state
  const [gameTopic, setGameTopic] = useState('');
  const [gameGenre, setGameGenre] = useState('RPG');
  const [gamePlatform, setGamePlatform] = useState('PC/Console');
  const [isGeneratingGame, setIsGeneratingGame] = useState(false);
  const [generatedGame, setGeneratedGame] = useState('');
  const [gameError, setGameError] = useState<string | null>(null);
  const gameAbortControllerRef = useRef<AbortController | null>(null);

  useEffect(() => {
    return () => {
      if (videoPollIntervalRef.current) {
        clearInterval(videoPollIntervalRef.current);
      }
    };
  }, []);

  const handleGenerateBook = async () => {
    if (!topic.trim()) {
      setBookError("Bitte gib ein Thema (Topic) ein.");
      return;
    }
    
    setBookError(null);
    setIsGeneratingBook(true);
    setGeneratedBook('');

    abortControllerRef.current = new AbortController();

    try {
      const franchiseContext = selectedFranchise ? `Franchise: ${selectedFranchise.name}. Beschreibung: ${selectedFranchise.description}. Lore Assets: ${JSON.stringify(selectedFranchise.assets)}` : '';
      const response = await fetch('/api/franchise-factory/generate-book', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ topic, genre, chapterCount, franchiseContext }),
        signal: abortControllerRef.current.signal
      });

      if (!response.ok) throw new Error('Fehler beim Starten der Generierung');

      if (response.body) {
        const reader = response.body.getReader();
        const decoder = new TextDecoder('utf-8');
        let done = false;
        let fullBookText = '';

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
                    setBookError(data.error);
                    done = true;
                    break;
                  }
                  if (data.text) {
                    fullBookText += data.text;
                    setGeneratedBook((prev) => prev + data.text);
                  }
                } catch (e) {
                  // Ignore
                }
              }
            }
          }
        }
        if (fullBookText) {
          handleAssetGenerated('Buch', `Buch: ${topic.substring(0, 30)}...`, fullBookText);
        }
      }
    } catch (err: any) {
      if (err.name !== 'AbortError') setBookError(err.message || "Unbekannter Fehler.");
    } finally {
      setIsGeneratingBook(false);
      abortControllerRef.current = null;
    }
  };

  const handleStopBook = () => {
    if (abortControllerRef.current) abortControllerRef.current.abort();
  };

  const handleGenerateGame = async () => {
    if (!gameTopic.trim()) {
      setGameError("Bitte gib ein Thema (Topic) ein.");
      return;
    }
    
    setGameError(null);
    setIsGeneratingGame(true);
    setGeneratedGame('');

    gameAbortControllerRef.current = new AbortController();

    try {
      const franchiseContext = selectedFranchise ? `Franchise: ${selectedFranchise.name}. Beschreibung: ${selectedFranchise.description}. Lore Assets: ${JSON.stringify(selectedFranchise.assets)}` : '';
      const response = await fetch('/api/franchise-factory/generate-game', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ topic: gameTopic, genre: gameGenre, platform: gamePlatform, franchiseContext }),
        signal: gameAbortControllerRef.current.signal
      });

      if (!response.ok) throw new Error('Fehler beim Starten der Generierung');

      if (response.body) {
        const reader = response.body.getReader();
        const decoder = new TextDecoder('utf-8');
        let done = false;

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
                    setGameError(data.error);
                    done = true;
                    break;
                  }
                  if (data.text) {
                    setGeneratedGame((prev) => prev + data.text);
                  }
                } catch (e) {
                  // Ignore
                }
              }
            }
          }
        }
      }
    } catch (err: any) {
      if (err.name !== 'AbortError') setGameError(err.message || "Unbekannter Fehler.");
    } finally {
      setIsGeneratingGame(false);
      gameAbortControllerRef.current = null;
    }
  };

  const handleStopGame = () => {
    if (gameAbortControllerRef.current) gameAbortControllerRef.current.abort();
  };

  const handleGenerateImage = async () => {
    if (!imagePrompt.trim()) {
      setImageError("Bitte gib einen Bild-Prompt ein.");
      return;
    }
    setImageError(null);
    setIsGeneratingImage(true);
    setGeneratedImageUrl('');

    try {
      const franchiseContext = selectedFranchise ? `Franchise: ${selectedFranchise.name}. Beschreibung: ${selectedFranchise.description}. Lore Assets: ${JSON.stringify(selectedFranchise.assets)}` : '';
      const promptWithContext = franchiseContext ? `${franchiseContext}\n\nBild-Prompt: ${imagePrompt}` : imagePrompt;
      const response = await fetch('/api/franchise-factory/generate-image', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: promptWithContext, aspectRatio: imageAspectRatio }),
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.error || 'Fehler beim Starten der Generierung');
      if (data.imageUrl) setGeneratedImageUrl(data.imageUrl);
    } catch (err: any) {
      setImageError(err.message || "Ein unbekannter Fehler ist aufgetreten.");
    } finally {
      setIsGeneratingImage(false);
    }
  };

  const handleGenerateVideo = async () => {
    if (!videoPrompt.trim()) {
      setVideoError("Bitte gib einen Video-Prompt ein.");
      return;
    }
    setVideoError(null);
    setIsGeneratingVideo(true);
    setGeneratedVideoUrl('');
    setVideoOperationName(null);
    setVideoPollingStatus('Starte Video-Generierung...');

    try {
      let finalPrompt = videoPrompt;
      if (videoFormat === 'movie') {
        finalPrompt = `Ganzteiliger Spielfilm, Trailer/Teaser Style: ${videoPrompt}. Cinematic lighting, highly detailed film composition, Hollywood style features.`;
      } else if (videoFormat === 'series') {
        finalPrompt = `TV-Serie Episoden Showcase: ${videoPrompt}. Episodic TV style, dramatic sequence, cinematic.`;
      }

      const franchiseContext = selectedFranchise ? `[Franchise Context: ${selectedFranchise.name} - ${selectedFranchise.description}] ` : '';
      finalPrompt = franchiseContext + finalPrompt;

      const response = await fetch('/api/franchise-factory/generate-video', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: finalPrompt, aspectRatio: videoAspectRatio }),
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
    } catch (err: any) {
      setVideoError(err.message);
    } finally {
      setIsGeneratingVideo(false);
      setVideoPollingStatus(null);
    }
  };

  const handleCreateStudio = () => {
    if (!newStudioName.trim()) return;
    const newStudio: Studio = {
      id: `STD-${Date.now()}`,
      name: newStudioName,
      franchises: []
    };
    setStudios([...studios, newStudio]);
    setSelectedStudioId(newStudio.id);
    setSelectedFranchiseId('');
    setNewStudioName('');
    setIsCreatingStudio(false);
  };

  const handleCreateFranchise = () => {
    if (!newFranchiseName.trim()) return;
    const newFr: Franchise = {
      id: `FRA-${Date.now()}`,
      name: newFranchiseName,
      description: newFranchiseDesc,
      assets: []
    };
    setStudios(studios.map(s => {
      if (s.id === selectedStudioId) {
        return { ...s, franchises: [...s.franchises, newFr] };
      }
      return s;
    }));
    setSelectedFranchiseId(newFr.id);
    setNewFranchiseName('');
    setNewFranchiseDesc('');
    setIsCreatingFranchise(false);
  };

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-[0_0_15px_rgba(6,182,212,0.2)]">
            <Factory className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
              Franchise Factory
            </h2>
            <p className="text-slate-400 font-light max-w-3xl leading-relaxed">
              Verwalte Studios und Franchises, um Konsistenz in der Produktion zu gewährleisten.
            </p>
          </div>
        </div>
      </div>

      {/* Studio & Franchise Selector Ribbon */}
      <div className="p-4 rounded-xl bg-black/40 border border-white/10 flex flex-wrap gap-6 items-end relative z-10">
        <div className="flex-1 min-w-[250px]">
          <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Aktuelles Studio</label>
          {isCreatingStudio ? (
            <div className="flex gap-2">
              <input 
                type="text" 
                value={newStudioName} 
                onChange={e => setNewStudioName(e.target.value)} 
                placeholder="Studio Name..." 
                className="bg-[#0b0f19] border border-white/10 text-white text-sm rounded-lg px-3 py-2 outline-none focus:border-cyan-500 flex-1"
              />
              <button onClick={handleCreateStudio} className="px-3 py-2 bg-cyan-600 hover:bg-cyan-500 text-white rounded-lg text-sm">Speichern</button>
              <button onClick={() => setIsCreatingStudio(false)} className="px-3 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-sm">Abbrechen</button>
            </div>
          ) : (
            <div className="flex gap-2">
              <select 
                value={selectedStudioId} 
                onChange={e => setSelectedStudioId(e.target.value)}
                className="bg-[#0b0f19] border border-white/10 text-white text-sm rounded-lg px-3 py-2 outline-none focus:border-cyan-500 flex-1"
              >
                {studios.map(s => (
                  <option key={s.id} value={s.id}>{s.name}</option>
                ))}
              </select>
              <button onClick={() => setIsCreatingStudio(true)} className="px-3 py-2 bg-slate-800 hover:bg-slate-700 text-cyan-400 rounded-lg text-sm whitespace-nowrap">
                + Studio
              </button>
            </div>
          )}
        </div>

        <div className="flex-1 min-w-[300px]">
          <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Ausgewähltes Franchise</label>
          {isCreatingFranchise ? (
            <div className="flex flex-col gap-2">
              <div className="flex gap-2">
                <input 
                  type="text" 
                  value={newFranchiseName} 
                  onChange={e => setNewFranchiseName(e.target.value)} 
                  placeholder="Franchise Name..." 
                  className="bg-[#0b0f19] border border-white/10 text-white text-sm rounded-lg px-3 py-2 outline-none focus:border-cyan-500 flex-1"
                />
              </div>
              <div className="flex gap-2">
                <input 
                  type="text" 
                  value={newFranchiseDesc} 
                  onChange={e => setNewFranchiseDesc(e.target.value)} 
                  placeholder="Kurze Beschreibung/Lore..." 
                  className="bg-[#0b0f19] border border-white/10 text-white text-sm rounded-lg px-3 py-2 outline-none focus:border-cyan-500 flex-1"
                />
                <button onClick={handleCreateFranchise} className="px-3 py-2 bg-cyan-600 hover:bg-cyan-500 text-white rounded-lg text-sm">Speichern</button>
                <button onClick={() => setIsCreatingFranchise(false)} className="px-3 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-sm">Abbrechen</button>
              </div>
            </div>
          ) : (
            <div className="flex gap-2">
              <select 
                value={selectedFranchiseId} 
                onChange={e => setSelectedFranchiseId(e.target.value)}
                className="bg-[#0b0f19] border border-white/10 text-white text-sm rounded-lg px-3 py-2 outline-none focus:border-cyan-500 flex-1"
                disabled={!selectedStudio || selectedStudio.franchises.length === 0}
              >
                {selectedStudio?.franchises.map(f => (
                  <option key={f.id} value={f.id}>{f.name}</option>
                ))}
              </select>
              <button 
                onClick={() => setIsCreatingFranchise(true)} 
                disabled={!selectedStudio}
                className="px-3 py-2 bg-slate-800 hover:bg-slate-700 text-cyan-400 rounded-lg text-sm whitespace-nowrap disabled:opacity-50"
              >
                + Franchise
              </button>
            </div>
          )}
        </div>
      </div>

      <div className="flex flex-wrap gap-4 border-b border-white/10 pb-4">
        <button
          onClick={() => setActiveTab('repositories')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'repositories'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Github className="w-4 h-4" />
          Repos & Wiki
        </button>
        <button
          onClick={() => setActiveTab('database')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'database'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Database className="w-4 h-4" />
          Franchise DB
        </button>
        <button
          onClick={() => setActiveTab('image')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'image'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <ImageIcon className="w-4 h-4" />
          KI Image Generator
        </button>
        <button
          onClick={() => setActiveTab('video')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'video'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <VideoIcon className="w-4 h-4" />
          KI Video Generator
        </button>
        <button
          onClick={() => setActiveTab('film')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'film'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Film className="w-4 h-4" />
          Film KI
        </button>
        <button
          onClick={() => setActiveTab('serie')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'serie'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <MonitorPlay className="w-4 h-4" />
          Serien KI
        </button>
        <button
          onClick={() => setActiveTab('book')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'book'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <BookOpen className="w-4 h-4" />
          AI Book Writer
        </button>
        <button
          onClick={() => setActiveTab('game')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'game'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Gamepad2 className="w-4 h-4" />
          KI Game Generator
        </button>
        <button
          onClick={() => setActiveTab('wiki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'wiki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <FileText className="w-4 h-4" />
          Franchise Wiki
        </button>
        <button
          onClick={() => setActiveTab('lore')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'lore'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Compass className="w-4 h-4" />
          Lore KI
        </button>
        <button
          onClick={() => setActiveTab('char_bios')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'char_bios'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <UserCircle className="w-4 h-4" />
          Charakter Biografien
        </button>
        <button
          onClick={() => setActiveTab('npc')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'npc'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Users className="w-4 h-4" />
          NPC KI
        </button>
        <button
          onClick={() => setActiveTab('world_design')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'world_design'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Map className="w-4 h-4" />
          World Design KI
        </button>
        <button
          onClick={() => setActiveTab('prompts')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'prompts'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Terminal className="w-4 h-4" />
          Prompts KI
        </button>
        <button
          onClick={() => setActiveTab('ambiente')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'ambiente'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <CloudFog className="w-4 h-4" />
          Ambiente KI
        </button>
        <button
          onClick={() => setActiveTab('timeline')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'timeline'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <History className="w-4 h-4" />
          Timeline KI
        </button>
        <button
          onClick={() => setActiveTab('animation')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'animation'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Film className="w-4 h-4" />
          Animation KI
        </button>
        <button
          onClick={() => setActiveTab('game_assets')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'game_assets'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <ImageIcons className="w-4 h-4" />
          Game Assets KI
        </button>
        <button
          onClick={() => setActiveTab('textures')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'textures'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <ImageIcon className="w-4 h-4" />
          Texturen KI
        </button>
        <button
          onClick={() => setActiveTab('mesh')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'mesh'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Hexagon className="w-4 h-4" />
          Mesh KI
        </button>
        <button
          onClick={() => setActiveTab('effect')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'effect'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Wand2 className="w-4 h-4" />
          Effect KI
        </button>
        <button
          onClick={() => setActiveTab('musik')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'musik'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Music className="w-4 h-4" />
          Musik KI
        </button>
        <button
          onClick={() => setActiveTab('sfx')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'sfx'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Volume2 className="w-4 h-4" />
          Sound Effekte KI
        </button>
        <button
          onClick={() => setActiveTab('cinematic')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'cinematic'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <MonitorPlay className="w-4 h-4" />
          Cinematic KI
        </button>
        <button
          onClick={() => setActiveTab('sync')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'sync'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Mic className="w-4 h-4" />
          Synchronisation KI
        </button>
        <button
          onClick={() => setActiveTab('av_sync')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'av_sync'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Activity className="w-4 h-4" />
          A/V Sync KI
        </button>
        <button
          onClick={() => setActiveTab('consistency')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'consistency'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <CheckCircle className="w-4 h-4" />
          Konsistent KI
        </button>
        <button
          onClick={() => setActiveTab('game_mechanics')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'game_mechanics'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Cog className="w-4 h-4" />
          Spiele Mechanik KI
        </button>
        <button
          onClick={() => setActiveTab('frontend')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'frontend'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Layout className="w-4 h-4" />
          Frontend KI
        </button>
        <button
          onClick={() => setActiveTab('backend')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'backend'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Server className="w-4 h-4" />
          Backend KI
        </button>
        <button
          onClick={() => setActiveTab('bewerbung_assets')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'bewerbung_assets'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Briefcase className="w-4 h-4" />
          Bewerbungs-Assets KI
        </button>
        <button
          onClick={() => setActiveTab('franchise_code')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'franchise_code'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Code className="w-4 h-4" />
          Franchise Code KI
        </button>
        <button
          onClick={() => setActiveTab('avatar_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'avatar_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <UserCircle className="w-4 h-4" />
          Avatar KI
        </button>
        <button
          onClick={() => setActiveTab('werbung_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'werbung_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Megaphone className="w-4 h-4" />
          Werbung KI
        </button>
        <button
          onClick={() => setActiveTab('sozialmedia_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'sozialmedia_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Share2 className="w-4 h-4" />
          Sozialmedia KI
        </button>
        <button
          onClick={() => setActiveTab('sozialmedia_avatar_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'sozialmedia_avatar_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Bot className="w-4 h-4" />
          Sozialmedia Avatar KI
        </button>
        <button
          onClick={() => setActiveTab('marketing_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'marketing_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <TrendingUp className="w-4 h-4" />
          Marketing KI
        </button>
        <button
          onClick={() => setActiveTab('roadmap_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'roadmap_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Milestone className="w-4 h-4" />
          Roadmap KI
        </button>
        <button
          onClick={() => setActiveTab('todo_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'todo_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <ListTodo className="w-4 h-4" />
          Todo KI
        </button>
        <button
          onClick={() => setActiveTab('inhaltsverzeichnis_ki')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'inhaltsverzeichnis_ki'
              ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <List className="w-4 h-4" />
          Inhaltsverzeichnis KI
        </button>
        <button
          onClick={() => setActiveTab('all_in_one')}
          className={`flex items-center gap-2 px-4 py-2 font-mono text-sm uppercase tracking-wider rounded-lg transition-colors ${
            activeTab === 'all_in_one'
              ? 'bg-purple-500/20 text-purple-400 border border-purple-500/30'
              : 'text-slate-400 hover:text-white hover:bg-white/5'
          }`}
        >
          <Workflow className="w-4 h-4" />
          All-in-One Pipeline
        </button>
      </div>

      {activeTab === 'repositories' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="p-6 rounded-2xl bg-[#0b0f19] border border-cyan-500/20 shadow-[0_0_20px_rgba(6,182,212,0.05)] flex flex-col gap-6">
            <div className="flex items-center gap-3 border-b border-white/10 pb-4">
              <Github className="w-5 h-5 text-cyan-400" />
              <h3 className="text-lg font-semibold text-white">Studio Layout & Repositories</h3>
            </div>
            
            <div className="flex flex-col gap-4">
              <div className="p-4 bg-black/40 border border-white/10 rounded-xl relative overflow-hidden">
                <div className="absolute top-0 left-0 w-1 h-full bg-cyan-500" />
                <h4 className="text-sm font-mono text-cyan-400 uppercase tracking-wider flex items-center gap-2 mb-2">
                  <Globe className="w-4 h-4" /> Studio Main Repository
                </h4>
                <p className="text-white font-medium mb-1">{selectedStudio?.name || 'Kein Studio ausgewählt'}</p>
                <div className="flex items-center gap-2">
                  <input 
                    type="text" 
                    value={selectedStudio?.mainRepository || ''} 
                    onChange={(e) => {
                      if (!selectedStudio) return;
                      setStudios(studios.map(s => s.id === selectedStudio.id ? { ...s, mainRepository: e.target.value } : s));
                    }}
                    placeholder="github.com/..."
                    className="flex-1 bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:border-cyan-500/50 font-mono"
                  />
                </div>
              </div>

              <div className="ml-8 border-l-2 border-white/10 pl-6 flex flex-col gap-4">
                {selectedStudio?.franchises.map(franchise => (
                  <div key={franchise.id} className={`p-4 bg-black/40 border rounded-xl relative ${selectedFranchiseId === franchise.id ? 'border-cyan-500/50 shadow-[0_0_15px_rgba(6,182,212,0.1)]' : 'border-white/10'}`}>
                    <div className="absolute top-1/2 -left-6 w-6 h-0.5 bg-white/10" />
                    <h4 className="flex items-center justify-between text-white font-medium mb-3">
                      <span className="flex items-center gap-2">
                        <GitBranch className="w-4 h-4 text-cyan-500" /> Sub-Repository: {franchise.name}
                      </span>
                    </h4>
                    
                    <div className="flex flex-col gap-3">
                      <div>
                        <label className="text-[10px] uppercase font-mono text-slate-400 mb-1 block">Data Repository (Assets/Lore)</label>
                        <input 
                          type="text" 
                          value={franchise.repository || ''} 
                          onChange={(e) => {
                            setStudios(studios.map(s => s.id === selectedStudio.id ? {
                              ...s, 
                              franchises: s.franchises.map(f => f.id === franchise.id ? { ...f, repository: e.target.value } : f)
                            } : s));
                          }}
                          placeholder="github.com/.../franchise"
                          className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:border-cyan-500/50 font-mono"
                        />
                      </div>
                      <div>
                        <label className="text-[10px] uppercase font-mono text-slate-400 mb-1 block">Wiki Repository / Docs</label>
                        <input 
                          type="text" 
                          value={franchise.wikiUrl || ''} 
                          onChange={(e) => {
                            setStudios(studios.map(s => s.id === selectedStudio.id ? {
                              ...s, 
                              franchises: s.franchises.map(f => f.id === franchise.id ? { ...f, wikiUrl: e.target.value } : f)
                            } : s));
                          }}
                          placeholder="github.com/.../franchise/wiki"
                          className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-slate-300 focus:outline-none focus:border-cyan-500/50 font-mono"
                        />
                      </div>
                    </div>
                  </div>
                ))}
                
                {(!selectedStudio?.franchises || selectedStudio.franchises.length === 0) && (
                  <div className="p-4 border border-dashed border-white/20 rounded-xl text-center text-slate-500 text-sm font-mono">
                    Keine Franchises vorhanden
                  </div>
                )}
              </div>
            </div>
          </div>

          <div className="p-6 rounded-2xl bg-[#0b0f19] border border-white/5 flex flex-col gap-6">
            <h3 className="text-lg font-semibold text-white border-b border-white/10 pb-4">Franchise Data Overview</h3>
            {selectedFranchise ? (
              <div className="flex flex-col gap-4">
                <div className="bg-cyan-500/10 border border-cyan-500/30 p-4 rounded-xl">
                  <h4 className="text-cyan-400 font-bold mb-1">{selectedFranchise.name}</h4>
                  <p className="text-slate-300 text-sm">{selectedFranchise.description}</p>
                </div>
                
                <div className="grid grid-cols-2 gap-4">
                  <div className="bg-black/40 border border-white/10 p-4 rounded-xl flex flex-col items-center justify-center text-center">
                    <Database className="w-6 h-6 text-indigo-400 mb-2" />
                    <span className="text-2xl font-bold text-white">{selectedFranchise.assets.length}</span>
                    <span className="text-[10px] uppercase font-mono text-slate-500">Lore Assets</span>
                  </div>
                  <div className="bg-black/40 border border-white/10 p-4 rounded-xl flex flex-col items-center justify-center text-center">
                    <FileText className="w-6 h-6 text-emerald-400 mb-2" />
                    <span className="text-sm font-bold text-white">
                      {selectedFranchise.wikiUrl ? 'Aktiv' : 'Fehlt'}
                    </span>
                    <span className="text-[10px] uppercase font-mono text-slate-500">Wiki Setup</span>
                  </div>
                </div>

                {selectedFranchise.wikiUrl && (
                   <div className="mt-4 flex items-center justify-between p-4 bg-black/60 rounded-xl border border-white/5">
                     <span className="text-slate-400 text-sm font-mono flex items-center gap-2">
                       <Globe className="w-4 h-4" /> Wiki is ready to sync
                     </span>
                     <button className="px-3 py-1.5 bg-white/10 hover:bg-white/20 border border-white/10 rounded-lg text-xs font-mono text-white transition-colors">
                       Open in AT-Explorer
                     </button>
                   </div>
                )}
              </div>
            ) : (
              <div className="text-slate-500 text-center py-10 font-mono text-sm">
                Bitte erstelle ein Franchise, um Daten zu sehen.
              </div>
            )}
          </div>
        </div>
      )}

      {activeTab === 'database' && (
      <div className="p-6 rounded-2xl bg-[#0b0f19] border border-white/5 shadow-[0_0_20px_rgba(6,182,212,0.05)] flex flex-col gap-6">
        <div className="flex items-center gap-3 border-b border-white/10 pb-4">
          <Database className="w-5 h-5 text-cyan-400" />
          <h3 className="text-lg font-semibold text-white">Franchise Assets Database</h3>
        </div>
        <p className="text-slate-400 mb-2">Hier ist eine Übersicht über existierende Franchise Assets in deinem ausgewählten Franchise ({selectedFranchise?.name || 'Keins'}). Diese Assets stellen eine konsistente Lore-Basis für alle Factory-KIs dar.</p>
        {selectedFranchise?.assets && selectedFranchise.assets.length > 0 ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {selectedFranchise.assets.map((asset, i) => (
              <div key={i} className="p-4 bg-black/40 border border-white/10 rounded-xl hover:border-cyan-500/50 transition-colors">
                <div className="text-[10px] uppercase font-mono text-cyan-500 mb-2 tracking-wider">{asset.tag} · {asset.id}</div>
                <h4 className="text-white font-medium mb-1">{asset.name}</h4>
                <p className="text-sm text-slate-400 leading-relaxed">{asset.desc}</p>
              </div>
            ))}
          </div>
        ) : (
          <div className="text-slate-500 font-mono text-sm">Keine Assets gefunden. Nutze die KI, um neue Assets zu generieren.</div>
        )}
      </div>
      )}

      {activeTab === 'image' && (
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="p-6 rounded-2xl bg-[#0b0f19] border border-cyan-500/20 shadow-[0_0_20px_rgba(6,182,212,0.05)] flex flex-col gap-6">
          <div className="flex items-center gap-3 border-b border-white/10 pb-4">
            <ImageIcon className="w-5 h-5 text-cyan-400" />
            <h3 className="text-lg font-semibold text-white">Image Generator</h3>
          </div>
          <div className="flex flex-col gap-4">
            <div>
              <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Bild-Prompt</label>
              <textarea
                value={imagePrompt}
                onChange={(e) => setImagePrompt(e.target.value)}
                placeholder="A serene mountain landscape, photorealistic, cinematic lighting..."
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
              {isGeneratingImage ? <><Loader2 className="w-4 h-4 animate-spin" /> Generiere...</> : <><ImageIcon className="w-4 h-4" /> Bild generieren</>}
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
               <ImageIcon className="w-10 h-10 opacity-20" />
               <p>Output Preview<br/>Bild wird hier angezeigt.</p>
            </div>
          )}
        </div>
      </div>
      )}

      {activeTab === 'video' && (
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="p-6 rounded-2xl bg-[#0b0f19] border border-cyan-500/20 shadow-[0_0_20px_rgba(6,182,212,0.05)] flex flex-col gap-6">
          <div className="flex items-center gap-3 border-b border-white/10 pb-4">
            <VideoIcon className="w-5 h-5 text-cyan-400" />
            <h3 className="text-lg font-semibold text-white">Filme & Serien KI (Veo)</h3>
          </div>
          <div className="flex flex-col gap-4">
            <div>
              <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Pitch / Video-Prompt</label>
              <textarea
                value={videoPrompt}
                onChange={(e) => setVideoPrompt(e.target.value)}
                placeholder="Pitch für einen Film oder eine Serie..."
                className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50 min-h-[100px] resize-y"
                disabled={isGeneratingVideo}
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Video Format</label>
                <select
                  value={videoFormat}
                  onChange={(e) => setVideoFormat(e.target.value)}
                  className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50"
                  disabled={isGeneratingVideo}
                >
                  <option value="short">Kurzclip / Szene</option>
                  <option value="movie">Ganzer Spielfilm (Trailer/Teaser Focus)</option>
                  <option value="series">TV-Serie (Episoden Konzept)</option>
                </select>
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
              {isGeneratingVideo ? <><Loader2 className="w-4 h-4 animate-spin" /> Verarbeiten...</> : <><VideoIcon className="w-4 h-4" /> Video generieren</>}
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
               <VideoIcon className="w-10 h-10 opacity-20" />
               <p>Output Preview<br/>Video wird hier angezeigt.</p>
            </div>
          )}
        </div>
      </div>
      )}

      {activeTab === 'book' && (
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="p-6 rounded-2xl bg-[#0b0f19] border border-cyan-500/20 shadow-[0_0_20px_rgba(6,182,212,0.05)] flex flex-col gap-6">
          <div className="flex items-center gap-3 border-b border-white/10 pb-4">
            <BookOpen className="w-5 h-5 text-cyan-400" />
            <h3 className="text-lg font-semibold text-white">AI Book Writer</h3>
          </div>

          <div className="flex flex-col gap-4">
            <div>
              <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Thema / Hauptidee</label>
              <textarea
                value={topic}
                onChange={(e) => setTopic(e.target.value)}
                placeholder="z.B. Der Aufstieg einer dezentralisierten KI in einer Megacity..."
                className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50 min-h-[100px] resize-y"
                disabled={isGeneratingBook}
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Genre & Stil</label>
                <select
                  value={genre}
                  onChange={(e) => setGenre(e.target.value)}
                  className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50"
                  disabled={isGeneratingBook}
                >
                  <option value="Sci-Fi">Sci-Fi / Cyberpunk</option>
                  <option value="Fantasy">Fantasy</option>
                  <option value="Thriller">Thriller</option>
                  <option value="Non-Fiction">Non-Fiction / Doku</option>
                  <option value="Cyber Noir">Cyber Noir</option>
                </select>
              </div>
              <div>
                <label className="block text-xs font-mono uppercase tracking-wider text-slate-400 mb-2">Kapitelanzahl (Zirka)</label>
                <input
                  type="number"
                  value={chapterCount}
                  onChange={(e) => setChapterCount(Number(e.target.value))}
                  min={1}
                  max={20}
                  className="w-full bg-black/40 border border-white/10 text-white text-sm rounded-xl p-3 outline-none focus:border-cyan-500/50"
                  disabled={isGeneratingBook}
                />
              </div>
            </div>

            {bookError && (
              <div className="p-3 bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg">
                {bookError}
              </div>
            )}

            <div className="pt-2 flex gap-3">
              <button
                onClick={handleGenerateBook}
                disabled={isGeneratingBook || !topic.trim()}
                className="flex-1 px-4 py-3 bg-cyan-600 hover:bg-cyan-500 text-white rounded-xl font-medium tracking-wide transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                {isGeneratingBook ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Generiere Buch...
                  </>
                ) : (
                  <>
                    <BookOpen className="w-4 h-4" />
                    Buch schreiben
                  </>
                )}
              </button>
              
              {isGeneratingBook && (
                <button
                  onClick={handleStopBook}
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
            {generatedBook ? (
              <div className="markdown-body prose prose-invert prose-cyan max-w-none prose-sm sm:prose-base">
                <Markdown>{generatedBook}</Markdown>
              </div>
            ) : (
              <div className="h-full flex flex-col items-center justify-center text-slate-600 font-light gap-3 text-center">
                <BookOpen className="w-10 h-10 opacity-20" />
                <p>Noch kein Buch generiert.<br/>Fülle die Details links aus und starte die KI.</p>
              </div>
            )}
            {isGeneratingBook && (
              <div className="flex items-center gap-2 text-cyan-500 mt-4 text-sm animate-pulse">
                <Loader2 className="w-4 h-4 animate-spin" />
                <em>Die KI tippt gerade...</em>
              </div>
            )}
          </div>
        </div>
      </div>
      )}

      {activeTab === 'serie' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Serien KI (Show Bible & Pitch)" assetTag="Serien KI (Show Bible & Pitch)" 
          description="Generiere Pitch-Dokumente, Episodenguides und Serien-Bibel für TV-Shows."
          promptPlaceholder="z.B. Eine Anthologie-Serie im Neo-Noir Sektor..."
          icon={<MonitorPlay className="w-5 h-5" />}
          additionalContextPrompt="Write a professional TV series pitch document including logline, show synopsis, main characters, and a breakdown of the first 3 episodes for:"
          buttonLabel="Serien-Pitch generieren"
        />
      )}

      {activeTab === 'film' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Film KI (Drehbuch & Pitch)" assetTag="Film KI (Drehbuch & Pitch)" 
          description="Erstelle Treatment-Dokumente, Skriptauszüge oder vollständige Film-Konzepte."
          promptPlaceholder="z.B. Ein Sci-Fi Heist-Movie auf einem fliegenden Casino-Schiff..."
          icon={<Film className="w-5 h-5" />}
          additionalContextPrompt="Write a professional feature film treatment. Include a logline, a 3-act structure summary, key themes, and visual tone description for:"
          buttonLabel="Film-Treatment generieren"
        />
      )}

      {activeTab === 'game' && (
        <AiGameEngineTab onAssetGenerated={handleAssetGenerated} selectedFranchise={selectedFranchise} />
      )}

      {activeTab === 'wiki' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Franchise Wiki Generator" assetTag="Franchise Wiki Generator" 
          description="Generiere detaillierte Wiki-Einträge für Hintergrundwissen, Lore, Fraktionen oder Technologien des Franchises."
          promptPlaceholder="z.B. Die Entstehungsgeschichte des Syndikats auf dem Mars..."
          icon={<FileText className="w-5 h-5" />}
          additionalContextPrompt="You are an expert lore master and wiki editor. Write a highly detailed, canonical wiki entry in markdown format about the following topic:"
          buttonLabel="Wiki generieren"
        />
      )}

      {activeTab === 'lore' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Lore KI" assetTag="Lore KI" 
          description="Erschaffe tiefgründige Weltbau-Konzepte, Mythen, Legenden und historische Epochen deines Franchises."
          promptPlaceholder="z.B. Die Mythologie der verlorenen Zivilisation der Sternenweber..."
          icon={<Compass className="w-5 h-5" />}
          additionalContextPrompt="You are a professional worldbuilder and lore-master. Create compelling, rich, and immersive worldbuilding lore for the following concept. Include details on history, culture, and key locations:"
          buttonLabel="Lore generieren"
        />
      )}

      {activeTab === 'char_bios' && (
        <AiCharacterBioTab onAssetGenerated={handleAssetGenerated} selectedFranchise={selectedFranchise} />
      )}

      {activeTab === 'npc' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="NPC KI Generator" assetTag="NPC KI Generator" 
          description="Erschaffe lebendige Nebencharaktere, Händler, Wachen oder Fraktions-Mitglieder."
          promptPlaceholder="z.B. Ein grimmiger Schmied mit einer geheimen Vergangenheit..."
          icon={<Users className="w-5 h-5" />}
          additionalContextPrompt="Create a detailed profile for a non-player character (NPC) or secondary character. Include their background, appearance, personality, goals, and potential dialogue lines for:"
          buttonLabel="NPC generieren"
        />
      )}

      {activeTab === 'world_design' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="World Design KI" assetTag="World Design KI" 
          description="Gestalte Biome, Städte, Architektur und Ökosysteme deines Franchises."
          promptPlaceholder="z.B. Eine fliegende Steampunk-Insel-Stadt..."
          icon={<Map className="w-5 h-5" />}
          additionalContextPrompt="Generate a detailed world design and location concept document. Describe the aesthetics, architecture, key locations, flora/fauna, and atmosphere for:"
          buttonLabel="World Design generieren"
        />
      )}

      {activeTab === 'prompts' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Professionelle Prompts KI" assetTag="Professionelle Prompts KI" 
          description="Generiere High-End Prompts für Midjourney, Stable Diffusion, oder ChatGPT."
          promptPlaceholder="z.B. Ein Prompt für einen Cyberpunk-Bösewicht..."
          icon={<Terminal className="w-5 h-5" />}
          additionalContextPrompt="Act as an expert prompt engineer. Generate 3 highly detailed, professional-grade prompts (for image generation tools like Midjourney or text AI) based on the following idea. Include technical camera settings, lighting, and style keywords:"
          buttonLabel="Prompts generieren"
        />
      )}

      {activeTab === 'ambiente' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Ambiente KI" assetTag="Ambiente KI" 
          description="Kreiere Stimmungsbilder, Wetterbeschreibungen und Raum-Atmosphären."
          promptPlaceholder="z.B. Ein regnerischer Neon-Marktplatz um Mitternacht..."
          icon={<CloudFog className="w-5 h-5" />}
          additionalContextPrompt="Create a highly descriptive, sensory-rich narrative outlining the atmosphere, ambient sounds, lighting, and overall mood for the following setting:"
          buttonLabel="Ambiente generieren"
        />
      )}

      {activeTab === 'timeline' && (
        <AiTimelineEngineTab onAssetGenerated={handleAssetGenerated} selectedFranchise={selectedFranchise} />
      )}

      {activeTab === 'mesh' && (
        <Ai3DRenderEngineTab onAssetGenerated={handleAssetGenerated} selectedFranchise={selectedFranchise} />
      )}

      {activeTab === 'animation' && (
        <AiAnimationEngineTab onAssetGenerated={handleAssetGenerated} selectedFranchise={selectedFranchise} />
      )}

      {activeTab === 'game_assets' && (
        <ImageGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Game Assets KI" assetTag="Game Assets KI" 
          description="Generiere Sprites, Icons, UI-Elemente oder Concept Art für Spiele."
          promptPlaceholder="z.B. Ein isometrischer Pixel-Art Cyber-Health-Potion..."
          icon={<ImageIcons className="w-5 h-5" />}
          additionalContextPrompt="Generate a clean game asset. White background, high quality, suitable for game development:"
          buttonLabel="Asset generieren"
        />
      )}

      {activeTab === 'textures' && (
        <ImageGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Texturen KI" assetTag="Texturen KI" 
          description="Erstelle nahtlose Materialien, PBR-Textur-Konzepte oder Sci-Fi Panels für 3D-Modelle."
          promptPlaceholder="z.B. Seamless sci-fi metal plating with neon accents, rust and scratches..."
          icon={<ImageIcon className="w-5 h-5" />}
          additionalContextPrompt="A seamless repeating texture material, flat front view layout:"
          buttonLabel="Textur generieren"
        />
      )}

      {activeTab === 'effect' && (
        <VideoGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Effect KI" assetTag="Effect KI" 
          description="Erstelle VFX (Visual Effects) Animationen, Sprite-Sheet Konzepte oder Partikel-Verhalten."
          promptPlaceholder="z.B. Eine magische blaue Flamme, Partikelexplosion..."
          icon={<Wand2 className="w-5 h-5" />}
        />
      )}

      {activeTab === 'musik' && (
        <AiAudioEngineTab onAssetGenerated={handleAssetGenerated} selectedFranchise={selectedFranchise} />
      )}

      {activeTab === 'sfx' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Sound Effekte (SFX) KI" assetTag="Sound Effekte (SFX) KI" 
          description="Generiere Konzepte für prägnante Soundeffekte, UI-Sounds und Atmosphären für das Franchise."
          promptPlaceholder="z.B. Laser-Schüsse, UI-Klicks, Ambient Drones..."
          icon={<Volume2 className="w-5 h-5" />}
          additionalContextPrompt="Create a detailed technical and creative sound effect concept for:"
          buttonLabel="SFX Konzept generieren"
        />
      )}

      {activeTab === 'cinematic' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Cinematic KI" assetTag="Cinematic KI" 
          description="Plane Kamerafahrten, Storyboards und Zwischensequenzen (Cutscenes)."
          promptPlaceholder="z.B. Ein epischer Intro-Skript für den Boss-Kampf..."
          icon={<MonitorPlay className="w-5 h-5" />}
          additionalContextPrompt="Write a highly detailed cinematic cutscene script including camera angles, lighting, and stage directions for:"
          buttonLabel="Cinematic generieren"
        />
      )}

      {activeTab === 'sync' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Synchronisation KI" assetTag="Synchronisation KI" 
          description="Generiere Dialog-Drehbücher mit Regieanweisungen für Sprecher und Lip-Sync-Hinweisen."
          promptPlaceholder="z.B. Ein hitziges Streitgespräch zwischen dem Hauptcharakter und dem Villain..."
          icon={<Mic className="w-5 h-5" />}
          additionalContextPrompt="Write a dialogue script for voice actors, including emotional cues, emphasis tags, and pacing notes for:"
          buttonLabel="Skript generieren"
        />
      )}

      {activeTab === 'av_sync' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="A/V Sync KI" assetTag="A/V Sync KI" 
          description="Gestalte präzise Timings, Cue Sheets und Audio-Visual Synchronization für Animationen und Film-Schnitte."
          promptPlaceholder="z.B. Eine Explosion passend zu einem Bass-Drop bei 120 BPM..."
          icon={<Activity className="w-5 h-5" />}
          additionalContextPrompt="Plan an audio-visual synchronization sheet. Include timestamps, animation cues, beat matching (BPM), and sound effect triggers for:"
          buttonLabel="Sync-Sheet generieren"
        />
      )}

      {activeTab === 'consistency' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Konsistenz KI" assetTag="Konsistenz KI" 
          description="Prüfe Universums-Regeln, Zeitlinien und Worldbuilding auf Logiklöcher (Plot Holes)."
          promptPlaceholder="z.B. Prüfe ob der Überlichtantrieb mit der Chronologie aus dem ersten Zeitalter übereinstimmt..."
          icon={<CheckCircle className="w-5 h-5" />}
          additionalContextPrompt="Act as a strict continuity editor and lore-master. Analyze the following scenario for plot holes, contradictions, and logical inconsistencies within its own ruleset, and suggest solutions:"
          buttonLabel="Konsistenz prüfen"
        />
      )}

      {activeTab === 'game_mechanics' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Spiele Mechanik KI" assetTag="Spiele Mechanik KI" 
          description="Entwirf Konzepte für innovative Spielemechaniken, Gameplay Loops und Balancing."
          promptPlaceholder="z.B. Ein Kampfsystem basierend auf Schwerkraft-Manipulation..."
          icon={<Cog className="w-5 h-5" />}
          additionalContextPrompt="Act as a Game Designer. Describe in technical detail a novel game mechanic, including player actions, system responses, balancing considerations, and mathematical progression logic for:"
          buttonLabel="Mechanik generieren"
        />
      )}

      {activeTab === 'frontend' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Frontend KI" assetTag="Frontend KI" 
          description="Architekturen für UI/UX Konzepte, React-Komponentenstrukturen und Frontend-Routing."
          promptPlaceholder="z.B. Ein futuristisches Dashboard für Raumschiffe..."
          icon={<Layout className="w-5 h-5" />}
          additionalContextPrompt="Act as a Lead Frontend Engineer. Architect a modern frontend solution for the following scenario, detailing the component structure, state management strategy, accessibility features, and styling approach:"
          buttonLabel="Frontend strukturieren"
        />
      )}

      {activeTab === 'backend' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Backend KI" assetTag="Backend KI" 
          description="Generiere Architektur-Pläne für Server, APIs, Datenbank-Schema und Microservices."
          promptPlaceholder="z.B. Ein skalierbares Multiplayer-Matchmaking System..."
          icon={<Server className="w-5 h-5" />}
          additionalContextPrompt="Act as a Lead Backend Engineer. Architect a robust, scalable backend system for the following requirements, detailing the database schema, API endpoints, caching strategy, and deployment infrastructure:"
          buttonLabel="Backend strukturieren"
        />
      )}

      {activeTab === 'bewerbung_assets' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Bewerbungs-Assets KI" assetTag="Bewerbungs-Assets KI" 
          description="Generiere professionelle Lebenslauf-Texte, Motivationsschreiben oder Pitch Decks für Studios & Publisher."
          promptPlaceholder="z.B. Ein Cover Letter als Lead Game Designer für ein Cyberpunk Projekt..."
          icon={<Briefcase className="w-5 h-5" />}
          additionalContextPrompt="Act as an expert career coach and copywriter for the game development and tech industry. Write a compelling, highly professional cover letter, pitch, or portfolio summary based on the following input:"
          buttonLabel="Asset generieren"
        />
      )}

      {activeTab === 'franchise_code' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Franchise Code KI" assetTag="Franchise Code KI" 
          description="Generiere Programmier-Codes, Skripte, Smart Contracts, API Interfaces oder Konfigurationsdateien für dein Franchise."
          promptPlaceholder="z.B. Ein Solidity Smart Contract für Franchise Assets oder ein React Dashboard-Entwurf..."
          icon={<Code className="w-5 h-5" />}
          additionalContextPrompt="Act as an expert Systems Architect and Senior Developer. Write high-quality, production-ready source code, configuration files, or software specifications. Include comments and clean formatting for the following request:"
          buttonLabel="Code generieren"
        />
      )}

      {activeTab === 'avatar_ki' && (
        <ImageGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Avatar KI" assetTag="Avatar KI" 
          description="Erstelle detailreiche Avatare und Charakterporträts für deine Franchise-Figuren."
          promptPlaceholder="z.B. Das Porträt eines Cyberpunk Spions mit brennend blauen Augen, Digital Painting-Stil..."
          icon={<UserCircle className="w-5 h-5" />}
          additionalContextPrompt="A stunning high-resolution character concept art avatar portrait, centered, highly detailed, expressive face, professional lighting:"
          buttonLabel="Avatar generieren"
        />
      )}

      {activeTab === 'werbung_ki' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Werbung KI" assetTag="Werbung KI" 
          description="Konstruiere fesselnde Werbeanzeigen (Ad Copy), Teaser, Pitch-Texte und Banner-Konzepte."
          promptPlaceholder="z.B. Ein viraler Werbespot-Vorschlag für die Kickstarter-Kampagne des Spiels..."
          icon={<Megaphone className="w-5 h-5" />}
          additionalContextPrompt="Act as an elite conversion copywriter and creative director. Write a high-converting, punchy advertisement copy, video script teaser, or marketing banner concepts for:"
          buttonLabel="Werbe-Asset generieren"
        />
      )}

      {activeTab === 'sozialmedia_ki' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Sozialmedia KI" assetTag="Sozialmedia KI" 
          description="Erschaffe optimierte Social Media Beiträge, Threads, Captions oder Ankündigungen für Twitter/X, Discord und Instagram."
          promptPlaceholder="z.B. Ein spannendes Twitter-Thread über die Lore unseres Franchises mit Hype-Faktor..."
          icon={<Share2 className="w-5 h-5" />}
          additionalContextPrompt="Act as a world-class Social Media Manager and community builder. Write engaging, native posts (tweets, Discord announcements, or Instagram captions) complete with relevant emojis, hooks, and hashtags for:"
          buttonLabel="Social Post generieren"
        />
      )}

      {activeTab === 'sozialmedia_avatar_ki' && (
        <ImageGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Sozialmedia Avatar KI" assetTag="Sozialmedia Avatar KI" 
          description="Generiere perfekt zugeschnittene Profilbilder, Logomuster oder Bannergrafiken für soziale Kanäle."
          promptPlaceholder="z.B. Minimalistisches, rundes Logo mit einem glühenden Neon-Symbol auf schwarzem Grund..."
          icon={<Bot className="w-5 h-5" />}
          additionalContextPrompt="A perfect circular-optimized minimalist logo or social media profile avatar. Clean graphic design style, vibrant color accents, dark background, centered masterfully:"
          buttonLabel="Social Avatar generieren"
        />
      )}

      {activeTab === 'marketing_ki' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Marketing KI" assetTag="Marketing KI" 
          description="Entwirf umfassende Marketingstrategien, Buyer Personas, SEO-Optimierungen und Kampagnen-Kalender."
          promptPlaceholder="z.B. Eine Go-To-Market Strategie für unseren Indie-Spiele-Release im Winter..."
          icon={<TrendingUp className="w-5 h-5" />}
          additionalContextPrompt="Act as a Senior Chief Marketing Officer (CMO). Develop a structured, actionable marketing strategy guide, audience analysis document, target demographics profile, or launch calendar for:"
          buttonLabel="Strategie generieren"
        />
      )}

      {activeTab === 'roadmap_ki' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Roadmap KI" assetTag="Roadmap KI" 
          description="Erstelle strukturierte Entwicklungs-Roadmaps, Meilenstein-Pläne und Release-Zyklen für das Franchise."
          promptPlaceholder="z.B. Eine 1-Jahres Roadmap für das Franchise mit Alpha, Beta & Release-Meilensteinen..."
          icon={<Milestone className="w-5 h-5" />}
          additionalContextPrompt="Act as a Lead Project Manager. Create a clean, chronologically organized development roadmap, outline milestones, sprints, key deliverables, and risk mitigation plans in markdown format for:"
          buttonLabel="Roadmap generieren"
        />
      )}

      {activeTab === 'todo_ki' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Todo KI" assetTag="Todo KI" 
          description="Erkenne und strukturiere offene Aufgaben, Backlogs und To-Do Checklisten für dein Entwickler-Team."
          promptPlaceholder="z.B. Ein detaillierter Entwicklungs-Backlog für die Implementierung des Kampfsystems..."
          icon={<ListTodo className="w-5 h-5" />}
          additionalContextPrompt="Act as an expert Scrum Master / Agile Lead. Convert the request into a beautifully organized checklist of actionable, prioritized tasks (Backlog) including Definition of Done (DoD) for each:"
          buttonLabel="Checkliste generieren"
        />
      )}

      {activeTab === 'inhaltsverzeichnis_ki' && (
        <TextGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="Inhaltsverzeichnis KI" assetTag="Inhaltsverzeichnis KI" 
          description="Erstelle strukturierte Inhaltsverzeichnisse (Table of Contents), Kapitelübersichten oder Dokument-Gliederungen."
          promptPlaceholder="z.B. Das Inhaltsverzeichnis für ein 300-seitiges Lore-Buch unseres Sci-Fi Universums..."
          icon={<List className="w-5 h-5" />}
          additionalContextPrompt="Act as a chief content editor and technical writer. Design a robust, deeply structured, logical Table of Contents or Document Outline in hierarchical markdown layout for:"
          buttonLabel="Gliederung generieren"
        />
      )}

      {activeTab === 'all_in_one' && (
        <PipelineGeneratorTab onAssetGenerated={handleAssetGenerated} 
          title="All-in-One Franchise Pipeline"
          description="Erstelle aus einer einfachen Idee automatisch Worldbuilding, Charaktere und ein Konzept-Bild nacheinander durch intelligente Agenten-Pipelines."
        />
      )}
    </div>
  );
}


