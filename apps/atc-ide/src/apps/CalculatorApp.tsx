import { Calculator } from 'lucide-react';
import React, { useState } from 'react';

export function CalculatorApp() {
  const [display, setDisplay] = useState('0');
  const [equation, setEquation] = useState('');

  const handleNum = (num: string) => {
    setDisplay(display === '0' ? num : display + num);
  };

  const handleOp = (op: string) => {
    setEquation(display + ' ' + op + ' ');
    setDisplay('0');
  };

  const calculate = () => {
    try {
      const res = eval(equation + display);
      setDisplay(String(res));
      setEquation('');
    } catch {
      setDisplay('Error');
    }
  };

  return (
    <div className="flex flex-col items-center justify-center p-4 font-sans h-full w-full bg-[#0c0c0e]">
      <div className="bg-slate-900 border border-white/10 rounded-xl p-4 w-64 shadow-2xl">
        <div className="flex items-center gap-2 mb-4 text-cyan-400">
           <Calculator className="w-5 h-5" />
           <span className="font-bold">ATC Calc</span>
        </div>
        <div className="bg-black/50 rounded-lg p-3 text-right mb-4">
          <div className="text-slate-500 text-xs h-4">{equation}</div>
          <div className="text-2xl text-slate-200 font-mono truncate">{display}</div>
        </div>
        <div className="grid grid-cols-4 gap-2">
          {['7','8','9','/'].map(btn => (
            <button key={btn} onClick={() => ['/','*','-','+'].includes(btn) ? handleOp(btn) : handleNum(btn)} className="p-3 bg-white/5 hover:bg-white/10 rounded-lg font-bold text-slate-300 transition-colors">{btn}</button>
          ))}
          {['4','5','6','*'].map(btn => (
             <button key={btn} onClick={() => ['/','*','-','+'].includes(btn) ? handleOp(btn) : handleNum(btn)} className="p-3 bg-white/5 hover:bg-white/10 rounded-lg font-bold text-slate-300 transition-colors">{btn}</button>
          ))}
          {['1','2','3','-'].map(btn => (
             <button key={btn} onClick={() => ['/','*','-','+'].includes(btn) ? handleOp(btn) : handleNum(btn)} className="p-3 bg-white/5 hover:bg-white/10 rounded-lg font-bold text-slate-300 transition-colors">{btn}</button>
          ))}
          <button onClick={() => { setDisplay('0'); setEquation(''); }} className="p-3 bg-red-500/20 text-red-400 hover:bg-red-500/30 rounded-lg font-bold transition-colors">C</button>
          <button onClick={() => handleNum('0')} className="p-3 bg-white/5 hover:bg-white/10 rounded-lg font-bold text-slate-300 transition-colors">0</button>
          <button onClick={calculate} className="p-3 bg-cyan-500/20 text-cyan-400 hover:bg-cyan-500/30 rounded-lg font-bold transition-colors">=</button>
          <button onClick={() => handleOp('+')} className="p-3 bg-white/5 hover:bg-white/10 rounded-lg font-bold text-slate-300 transition-colors">+</button>
        </div>
      </div>
    </div>
  );
}
