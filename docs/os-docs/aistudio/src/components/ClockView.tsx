import React, { useState, useEffect } from 'react';
import { Clock } from 'lucide-react';

export function ClockView() {
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const timer = setInterval(() => setTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  const hours = time.getHours().toString().padStart(2, '0');
  const minutes = time.getMinutes().toString().padStart(2, '0');
  const seconds = time.getSeconds().toString().padStart(2, '0');

  const secondDegrees = (time.getSeconds() / 60) * 360;
  const minuteDegrees = ((time.getMinutes() + time.getSeconds() / 60) / 60) * 360;
  const hourDegrees = ((time.getHours() % 12 + time.getMinutes() / 60) / 12) * 360;

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200 justify-center items-center p-6">
      <div className="bg-[#090b14] border border-white/10 rounded-2xl p-8 w-full max-w-md shadow-2xl flex flex-col items-center">
        <div className="flex items-center gap-2 mb-10 text-atc-cyan self-start w-full justify-between">
          <div className="flex items-center gap-2">
            <Clock className="w-5 h-5" />
            <h2 className="text-sm font-bold tracking-widest uppercase">World Clock</h2>
          </div>
          <span className="text-xs text-slate-500 font-mono">UTC {time.getTimezoneOffset() / -60 < 0 ? '-' : '+'}{Math.abs(time.getTimezoneOffset() / 60)}</span>
        </div>

        {/* Analog Clock */}
        <div className="relative w-48 h-48 rounded-full border-[4px] border-white/5 bg-black/40 shadow-inner flex items-center justify-center mb-10">
          {/* Hour markers */}
          {Array.from({ length: 12 }).map((_, i) => (
            <div 
              key={i} 
              className="absolute w-full h-full p-2"
              style={{ transform: `rotate(${i * 30}deg)` }}
            >
              <div className="w-1 h-3 bg-white/20 mx-auto rounded-full" />
            </div>
          ))}

          {/* Hands */}
          <div 
            className="absolute w-1.5 h-16 bg-slate-400 rounded-full origin-bottom"
            style={{ transform: `translateY(-50%) rotate(${hourDegrees}deg)` }}
          />
          <div 
            className="absolute w-1 h-20 bg-slate-300 rounded-full origin-bottom"
            style={{ transform: `translateY(-50%) rotate(${minuteDegrees}deg)` }}
          />
          <div 
            className="absolute w-0.5 h-24 bg-atc-cyan rounded-full origin-bottom"
            style={{ transform: `translateY(-50%) rotate(${secondDegrees}deg)` }}
          />
          
          {/* Center dot */}
          <div className="absolute w-3 h-3 bg-atc-cyan rounded-full shadow-[0_0_10px_rgba(6,182,212,0.5)]" />
        </div>

        {/* Digital display */}
        <div className="text-5xl font-bold tracking-tighter text-white mb-2 flex items-baseline">
          {hours}<span className="text-slate-500 mx-1 animate-pulse">:</span>{minutes}<span className="text-2xl text-atc-cyan ml-2 font-mono font-medium">{seconds}</span>
        </div>
        <div className="text-sm text-slate-400 uppercase tracking-widest">
          {time.toLocaleDateString(undefined, { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}
        </div>
      </div>
    </div>
  );
}
