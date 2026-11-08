import React, { useState } from 'react';
import { Calculator } from 'lucide-react';
import { evaluate } from 'mathjs';

export function CalculatorView() {
  const [display, setDisplay] = useState('0');
  const [equation, setEquation] = useState('');

  const handleNum = (num: string) => {
    setDisplay(display === '0' ? num : display + num);
  };

  const handleOp = (op: string) => {
    setEquation(display + ' ' + op + ' ');
    setDisplay('0');
  };

  const handleEqual = () => {
    try {
      const sanitized = (equation + display).replace(/[^-()\d/*+.]/g, '');
      const result = evaluate(sanitized);
      setDisplay(String(result));
      setEquation('');
    } catch {
      setDisplay('Error');
    }
  };

  const clear = () => {
    setDisplay('0');
    setEquation('');
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200 justify-center items-center p-6">
      <div className="bg-[#090b14] border border-white/10 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
        <div className="flex items-center gap-2 mb-6 text-atc-cyan">
          <Calculator className="w-5 h-5" />
          <h2 className="text-sm font-bold tracking-widest uppercase">ATC-Calc</h2>
        </div>
        
        <div className="bg-black/60 border border-white/5 rounded-xl p-4 mb-6 text-right">
          <div className="text-xs text-slate-500 min-h-[16px]">{equation}</div>
          <div className="text-4xl font-mono text-white tracking-tight truncate">{display}</div>
        </div>

        <div className="grid grid-cols-4 gap-3">
          <button onClick={clear} className="col-span-2 py-3 rounded-xl bg-red-500/10 text-red-500 font-bold hover:bg-red-500/20 transition-colors">AC</button>
          <button onClick={() => setDisplay(String(-parseFloat(display)))} className="py-3 rounded-xl bg-white/5 text-slate-300 font-bold hover:bg-white/10 transition-colors">+/-</button>
          <button onClick={() => handleOp('/')} className="py-3 rounded-xl bg-atc-cyan/20 text-atc-cyan font-bold hover:bg-atc-cyan/30 transition-colors">÷</button>

          {[7, 8, 9].map(n => (
            <button key={n} onClick={() => handleNum(String(n))} className="py-3 rounded-xl bg-white/5 text-white font-bold hover:bg-white/10 transition-colors">{n}</button>
          ))}
          <button onClick={() => handleOp('*')} className="py-3 rounded-xl bg-atc-cyan/20 text-atc-cyan font-bold hover:bg-atc-cyan/30 transition-colors">×</button>

          {[4, 5, 6].map(n => (
            <button key={n} onClick={() => handleNum(String(n))} className="py-3 rounded-xl bg-white/5 text-white font-bold hover:bg-white/10 transition-colors">{n}</button>
          ))}
          <button onClick={() => handleOp('-')} className="py-3 rounded-xl bg-atc-cyan/20 text-atc-cyan font-bold hover:bg-atc-cyan/30 transition-colors">-</button>

          {[1, 2, 3].map(n => (
            <button key={n} onClick={() => handleNum(String(n))} className="py-3 rounded-xl bg-white/5 text-white font-bold hover:bg-white/10 transition-colors">{n}</button>
          ))}
          <button onClick={() => handleOp('+')} className="py-3 rounded-xl bg-atc-cyan/20 text-atc-cyan font-bold hover:bg-atc-cyan/30 transition-colors">+</button>

          <button onClick={() => handleNum('0')} className="col-span-2 py-3 rounded-xl bg-white/5 text-white font-bold hover:bg-white/10 transition-colors">0</button>
          <button onClick={() => handleNum('.')} className="py-3 rounded-xl bg-white/5 text-white font-bold hover:bg-white/10 transition-colors">.</button>
          <button onClick={handleEqual} className="py-3 rounded-xl bg-atc-cyan text-slate-900 font-bold hover:bg-atc-cyan/90 transition-colors">=</button>
        </div>
      </div>
    </div>
  );
}
