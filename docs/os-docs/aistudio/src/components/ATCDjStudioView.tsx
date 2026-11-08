import React, { useState, useEffect, useRef } from 'react';
import { Play, Pause, Save, Headphones, Disc3 } from 'lucide-react';

// --- Web Audio Engine ---

function generateLoopA(ctx: AudioContext, bpm: number = 128): AudioBuffer {
  const beatLen = 60 / bpm;
  const loopLen = beatLen * 4;
  const buffer = ctx.createBuffer(1, ctx.sampleRate * loopLen, ctx.sampleRate);
  const data = buffer.getChannelData(0);
  
  for (let b = 0; b < 4; b++) {
    const startIdx = Math.floor(b * beatLen * ctx.sampleRate);
    for (let i = 0; i < ctx.sampleRate * 0.5; i++) {
      const t = i / ctx.sampleRate;
      if (startIdx + i < data.length) {
        const freq = Math.max(40, 150 * Math.exp(-t * 20)); // Kick
        data[startIdx + i] += Math.sin(2 * Math.PI * freq * t) * Math.exp(-t * 5);
      }
    }
  }
  for (let b = 0; b < 4; b++) {
    const startIdx = Math.floor((b + 0.5) * beatLen * ctx.sampleRate);
    for (let i = 0; i < ctx.sampleRate * 0.2; i++) {
      const t = i / ctx.sampleRate;
      if (startIdx + i < data.length) {
        data[startIdx + i] += (Math.random() * 2 - 1) * Math.exp(-t * 15) * 0.2; // Hat
      }
    }
  }
  return buffer;
}

function generateLoopB(ctx: AudioContext, bpm: number = 128): AudioBuffer {
  const beatLen = 60 / bpm;
  const loopLen = beatLen * 4;
  const buffer = ctx.createBuffer(1, ctx.sampleRate * loopLen, ctx.sampleRate);
  const data = buffer.getChannelData(0);
  
  const notes = [36, 0, 36, 39, 0, 36, 43, 0];
  const stepLen = beatLen / 2;
  for (let s = 0; s < 8; s++) {
    if (notes[s] === 0) continue;
    const startIdx = Math.floor(s * stepLen * ctx.sampleRate);
    const freq = 440 * Math.pow(2, (notes[s] - 69) / 12);
    for (let i = 0; i < ctx.sampleRate * 0.3; i++) {
      const t = i / ctx.sampleRate;
      if (startIdx + i < data.length) {
        let val = 0;
        for (let k = 1; k <= 3; k++) val += Math.sin(2 * Math.PI * freq * k * t) / k;
        data[startIdx + i] += val * Math.exp(-t * 6) * 0.4;
      }
    }
  }
  return buffer;
}

class DeckAudio {
  ctx: AudioContext;
  outNode: GainNode;
  
  lowEQ: BiquadFilterNode;
  midEQ: BiquadFilterNode;
  highEQ: BiquadFilterNode;
  filter: BiquadFilterNode;
  
  buffer: AudioBuffer;
  source: AudioBufferSourceNode | null = null;
  
  constructor(ctx: AudioContext, destination: AudioNode, buffer: AudioBuffer) {
    this.ctx = ctx;
    this.buffer = buffer;
    this.outNode = ctx.createGain();
    
    this.lowEQ = ctx.createBiquadFilter();
    this.lowEQ.type = 'lowshelf';
    this.lowEQ.frequency.value = 320;
    
    this.midEQ = ctx.createBiquadFilter();
    this.midEQ.type = 'peaking';
    this.midEQ.frequency.value = 1000;
    this.midEQ.Q.value = 0.5;
    
    this.highEQ = ctx.createBiquadFilter();
    this.highEQ.type = 'highshelf';
    this.highEQ.frequency.value = 3200;

    this.filter = ctx.createBiquadFilter();
    this.filter.type = 'lowpass';
    this.filter.frequency.value = 20000;
    
    // Audio graph
    this.lowEQ.connect(this.midEQ);
    this.midEQ.connect(this.highEQ);
    this.highEQ.connect(this.filter);
    this.filter.connect(this.outNode);
    this.outNode.connect(destination);
  }

  setGain(val: number) {
    this.outNode.gain.setTargetAtTime(val, this.ctx.currentTime, 0.05);
  }
  
  setEQ(type: 'low' | 'mid' | 'high', val: number) {
    if (type === 'low') this.lowEQ.gain.setTargetAtTime(val, this.ctx.currentTime, 0.05);
    if (type === 'mid') this.midEQ.gain.setTargetAtTime(val, this.ctx.currentTime, 0.05);
    if (type === 'high') this.highEQ.gain.setTargetAtTime(val, this.ctx.currentTime, 0.05);
  }

  setFilter(val: number) { 
    if (val < 0) {
      this.filter.type = 'lowpass';
      this.filter.frequency.setTargetAtTime(20000 * Math.pow(200 / 20000, -val / 100), this.ctx.currentTime, 0.05);
    } else if (val > 0) {
      this.filter.type = 'highpass';
      this.filter.frequency.setTargetAtTime(20 * Math.pow(2000 / 20, val / 100), this.ctx.currentTime, 0.05);
    } else {
      this.filter.type = 'lowpass';
      this.filter.frequency.setTargetAtTime(20000, this.ctx.currentTime, 0.05);
    }
  }

  play() {
    this.ctx.resume();
    this.source = this.ctx.createBufferSource();
    this.source.buffer = this.buffer;
    this.source.loop = true;
    this.source.connect(this.lowEQ);
    this.source.start();
  }

  stop() {
    if (this.source) {
      this.source.stop();
      this.source.disconnect();
      this.source = null;
    }
  }
}

class AudioEngine {
  ctx: AudioContext;
  masterGain: GainNode;
  mediaDest: MediaStreamAudioDestinationNode | null = null;
  mediaRecorder: MediaRecorder | null = null;
  recordedChunks: BlobPart[] = [];
  
  deckA: DeckAudio;
  deckB: DeckAudio;
  
  onRecordingComplete: (blob: Blob) => void = () => {};

  constructor() {
    this.ctx = new AudioContext();
    this.masterGain = this.ctx.createGain();
    
    if (typeof this.ctx.createMediaStreamDestination === 'function') {
      this.mediaDest = this.ctx.createMediaStreamDestination();
      this.masterGain.connect(this.mediaDest);
    }
    this.masterGain.connect(this.ctx.destination);

    this.deckA = new DeckAudio(this.ctx, this.masterGain, generateLoopA(this.ctx, 128));
    this.deckB = new DeckAudio(this.ctx, this.masterGain, generateLoopB(this.ctx, 128));
    
    this.setCrossfader(50);
  }

  startRecording() {
    if (!this.mediaDest) return false;
    this.recordedChunks = [];
    try {
      this.mediaRecorder = new MediaRecorder(this.mediaDest.stream);
      this.mediaRecorder.ondataavailable = (e) => {
        if (e.data.size > 0) this.recordedChunks.push(e.data);
      };
      this.mediaRecorder.onstop = () => {
         const blob = new Blob(this.recordedChunks, { type: 'audio/webm' });
         this.onRecordingComplete(blob);
      };
      this.mediaRecorder.start();
      return true;
    } catch (e) {
      return false;
    }
  }

  stopRecording() {
    if (this.mediaRecorder && this.mediaRecorder.state !== 'inactive') {
      this.mediaRecorder.stop();
    }
  }

  setCrossfader(value: number) { 
    // equal power crossfade
    const x = value / 100;
    const gainA = Math.cos(x * 0.5 * Math.PI);
    const gainB = Math.cos((1 - x) * 0.5 * Math.PI);
    this.deckA.setGain(gainA);
    this.deckB.setGain(gainB);
  }
}

// --- UI Components ---

function Knob({ label, value, min, max, onChange }: { label: string, value: number, min: number, max: number, onChange: (v: number) => void }) {
  const [isDragging, setIsDragging] = useState(false);
  const startY = useRef(0);
  const startVal = useRef(0);

  const handlePointerDown = (e: React.PointerEvent) => {
    if (e.target instanceof HTMLElement) {
      e.target.setPointerCapture(e.pointerId);
    }
    setIsDragging(true);
    startY.current = e.clientY;
    startVal.current = value;
  };

  const handlePointerMove = (e: React.PointerEvent) => {
    if (!isDragging) return;
    const dy = startY.current - e.clientY;
    const range = max - min;
    const delta = (dy * range) / 150; 
    let newVal = startVal.current + delta;
    newVal = Math.max(min, Math.min(max, newVal));
    onChange(newVal);
  };

  const handlePointerUp = (e: React.PointerEvent) => {
    if (e.target instanceof HTMLElement) {
      e.target.releasePointerCapture(e.pointerId);
    }
    setIsDragging(false);
  };

  const percent = (value - min) / (max - min);
  const rotation = -135 + percent * 270;

  return (
    <div className="flex flex-col items-center gap-2">
       <div 
         className="relative w-12 h-12 rounded-full bg-slate-900 border-2 border-slate-700 shadow-xl flex items-center justify-center cursor-ns-resize group hover:border-slate-500 transition-colors"
         onPointerDown={handlePointerDown}
         onPointerMove={handlePointerMove}
         onPointerUp={handlePointerUp}
         onPointerCancel={handlePointerUp}
         onDoubleClick={() => onChange(0)}
         style={{ touchAction: 'none' }}
       >
          <div className="w-full h-full rounded-full absolute" style={{ transform: `rotate(${rotation}deg)` }}>
             <div className="absolute top-1 left-1/2 -translate-x-1/2 w-1 h-2.5 bg-white/70 rounded-full group-hover:bg-white transition-colors" />
          </div>
       </div>
       <span className="text-[10px] font-bold text-slate-400 tracking-wider uppercase select-none">{label}</span>
    </div>
  );
}

export function ATCDjStudioView() {
  const engineRef = useRef<AudioEngine | null>(null);
  const [isRecording, setIsRecording] = useState(false);
  const [crossfader, setCrossfader] = useState(50);
  
  const [deckA, setDeckA] = useState({ isPlaying: false, high: 0, mid: 0, low: 0, filter: 0 });
  const [deckB, setDeckB] = useState({ isPlaying: false, high: 0, mid: 0, low: 0, filter: 0 });

  useEffect(() => {
    return () => {
      if (engineRef.current) {
        engineRef.current.deckA.stop();
        engineRef.current.deckB.stop();
        engineRef.current.ctx.close();
      }
    };
  }, []);

  const getEngine = () => {
    if (!engineRef.current) {
      engineRef.current = new AudioEngine();
      engineRef.current.setCrossfader(crossfader);
    }
    return engineRef.current;
  };

  const togglePlay = (side: 'A' | 'B') => {
    const engine = getEngine();
    const isA = side === 'A';
    const deckState = isA ? deckA : deckB;
    const setState = isA ? setDeckA : setDeckB as any;
    const engineDeck = isA ? engine.deckA : engine.deckB;
    
    if (deckState.isPlaying) {
      engineDeck.stop();
      setState((prev: any) => ({ ...prev, isPlaying: false }));
    } else {
      engineDeck.play();
      setState((prev: any) => ({ ...prev, isPlaying: true }));
    }
  };

  const updateEQ = (side: 'A' | 'B', type: 'high'|'mid'|'low'|'filter', val: number) => {
    const engine = getEngine();
    const isA = side === 'A';
    const setState = isA ? setDeckA : setDeckB as any;
    const engineDeck = isA ? engine.deckA : engine.deckB;
    
    setState((prev: any) => ({ ...prev, [type]: val }));
    
    if (type === 'filter') {
       engineDeck.setFilter(val);
    } else {
       engineDeck.setEQ(type, val);
    }
  };

  const handleCrossfade = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = parseInt(e.target.value);
    setCrossfader(val);
    if (engineRef.current) engineRef.current.setCrossfader(val);
  };

  const toggleRecording = () => {
    const engine = getEngine();
    if (isRecording) {
      engine.stopRecording();
      setIsRecording(false);
    } else {
      engine.onRecordingComplete = (blob) => {
         const url = URL.createObjectURL(blob);
         const a = document.createElement('a');
         a.style.display = 'none';
         a.href = url;
         a.download = `ATC_DJ_Mix_${Date.now()}.webm`;
         document.body.appendChild(a);
         a.click();
         setTimeout(() => URL.revokeObjectURL(url), 100);
      };
      if (engine.startRecording()) {
         setIsRecording(true);
      }
    }
  };

  const renderDeck = (side: 'A' | 'B', state: any) => (
    <div className="flex-1 bg-black/40 border border-white/5 rounded-2xl p-6 flex flex-col gap-6 shadow-[inset_0_0_50px_rgba(0,0,0,0.5)]">
      <div className="flex justify-between items-start">
        <div>
          <h3 className="text-xl font-black text-white">{side === 'A' ? 'Tech House Drums' : 'Acid Bass'}</h3>
          <p className="text-indigo-400 font-medium">Virtual Track {side}</p>
        </div>
        <div className="text-right">
          <div className="text-2xl font-mono font-bold text-emerald-400">128.0</div>
          <p className="text-xs text-slate-500 font-mono">BPM</p>
        </div>
      </div>

      <div className="flex flex-1 items-center justify-center py-4">
         <div className="relative flex items-center justify-center w-40 h-40 md:w-56 md:h-56">
          <div className={`absolute inset-0 rounded-full border-[6px] ${state.isPlaying ? 'border-indigo-500 shadow-[0_0_40px_rgba(99,102,241,0.4)]' : 'border-slate-800'} bg-black flex items-center justify-center transition-colors`}>
            <div className={`absolute inset-1 rounded-full border-2 border-white/5 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-zinc-800 via-zinc-950 to-black ${state.isPlaying ? 'animate-[spin_2s_linear_infinite]' : ''}`}>
               <div className="absolute inset-0 m-auto w-1/3 h-1/3 bg-indigo-600/20 rounded-full flex items-center justify-center border border-indigo-500/30">
                  <Disc3 className="w-1/2 h-1/2 text-indigo-400 opacity-50" />
               </div>
            </div>
          </div>
        </div>
      </div>

      <div className="flex items-center justify-between">
        <button 
            className={`w-16 h-16 rounded-full flex items-center justify-center border-2 transition-all ${state.isPlaying ? 'bg-indigo-500/20 border-indigo-500 text-indigo-400 shadow-[0_0_20px_rgba(99,102,241,0.5)]' : 'bg-white/5 border-white/10 text-slate-300 hover:bg-white/10'}`}
            onClick={() => togglePlay(side)}
        >
            {state.isPlaying ? <Pause className="w-8 h-8" /> : <Play className="w-8 h-8 ml-1" />}
        </button>
      </div>
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-[#030408] font-sans text-slate-200 select-none">
      <div className="flex items-center justify-between p-4 px-6 border-b border-white/5 bg-black/40">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-indigo-500/20 text-indigo-400 rounded-xl border border-indigo-500/30">
            <Headphones className="w-6 h-6" />
          </div>
          <div>
            <h1 className="text-xl font-black tracking-tighter text-white">ATC-DjStudio Pro</h1>
            <p className="text-xs text-indigo-400/80 font-mono tracking-widest uppercase">WebAudio Mixing Engine</p>
          </div>
        </div>
        <div>
           <button 
             onClick={toggleRecording}
             className={`flex items-center gap-2 px-4 py-2 rounded-xl text-sm font-bold transition-all border ${isRecording ? 'bg-red-500/20 border-red-500/50 text-red-400 animate-pulse' : 'bg-white/5 border-white/10 text-slate-300 hover:bg-white/10 hover:text-white'}`}
           >
             {isRecording ? <><div className="w-2 h-2 rounded-full bg-red-400" /> REC</> : <><Save className="w-4 h-4" /> Export Mix</>}
           </button>
        </div>
      </div>

      <div className="flex-1 flex flex-col lg:flex-row p-6 gap-6 overflow-y-auto custom-scrollbar">
        {renderDeck('A', deckA)}

        <div className="w-full lg:w-96 bg-gradient-to-b from-slate-900 to-black border border-white/10 rounded-2xl p-6 flex flex-col gap-6 shadow-2xl relative">
            <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px)] bg-[size:40px_100%] rounded-2xl pointer-events-none" />
            
            <div className="flex flex-col gap-6 z-10 flex-1 justify-center">
               <div className="flex justify-between px-4">
                  <Knob label="High" value={deckA.high} min={-24} max={24} onChange={(v) => updateEQ('A', 'high', v)} />
                  <Knob label="High" value={deckB.high} min={-24} max={24} onChange={(v) => updateEQ('B', 'high', v)} />
               </div>
               <div className="flex justify-between px-4">
                  <Knob label="Mid" value={deckA.mid} min={-24} max={24} onChange={(v) => updateEQ('A', 'mid', v)} />
                  <Knob label="Mid" value={deckB.mid} min={-24} max={24} onChange={(v) => updateEQ('B', 'mid', v)} />
               </div>
               <div className="flex justify-between px-4">
                  <Knob label="Low" value={deckA.low} min={-24} max={24} onChange={(v) => updateEQ('A', 'low', v)} />
                  <Knob label="Low" value={deckB.low} min={-24} max={24} onChange={(v) => updateEQ('B', 'low', v)} />
               </div>
               <div className="flex justify-between px-4 mt-2">
                  <Knob label="Filter" value={deckA.filter} min={-100} max={100} onChange={(v) => updateEQ('A', 'filter', v)} />
                  <Knob label="Filter" value={deckB.filter} min={-100} max={100} onChange={(v) => updateEQ('B', 'filter', v)} />
               </div>
            </div>

            <div className="h-16 w-full bg-black/60 rounded-xl flex items-center px-6 relative border border-white/10 z-10">
               <div className="w-full h-1 bg-white/5 rounded-full relative pointer-events-none">
                  <div className="absolute top-1/2 left-1/2 -translate-x-1/2 h-4 w-px bg-white/20" />
               </div>
               <input 
                 type="range" min="0" max="100" value={crossfader} onChange={handleCrossfade}
                 className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full px-6 appearance-none bg-transparent cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-10 [&::-webkit-slider-thumb]:h-6 [&::-webkit-slider-thumb]:bg-slate-300 [&::-webkit-slider-thumb]:rounded-sm [&::-webkit-slider-thumb]:shadow-lg hover:[&::-webkit-slider-thumb]:bg-white"
               />
               <span className="absolute bottom-2 left-4 text-[9px] font-bold text-slate-500 pointer-events-none">A</span>
               <span className="absolute bottom-2 right-4 text-[9px] font-bold text-slate-500 pointer-events-none">B</span>
            </div>
        </div>

        {renderDeck('B', deckB)}
      </div>
    </div>
  );
}
