import React, { useState } from 'react';
import { Calendar as CalendarIcon, ChevronLeft, ChevronRight } from 'lucide-react';

export function CalendarView() {
  const [currentDate, setCurrentDate] = useState(new Date());

  const daysInMonth = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 0).getDate();
  const firstDay = new Date(currentDate.getFullYear(), currentDate.getMonth(), 1).getDay();

  const prevMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() - 1, 1));
  };

  const nextMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 1));
  };

  const monthNames = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
  const dayNames = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200 justify-center items-center p-6">
      <div className="bg-[#090b14] border border-white/10 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-2 text-atc-cyan">
            <CalendarIcon className="w-5 h-5" />
            <h2 className="text-sm font-bold tracking-widest uppercase">ATC-Calendar</h2>
          </div>
          <div className="text-xs text-slate-400 font-mono">
            {new Date().toLocaleDateString(undefined, { dateStyle: 'medium' })}
          </div>
        </div>

        <div className="flex items-center justify-between mb-4 bg-black/40 rounded-xl p-2 border border-white/5">
          <button onClick={prevMonth} className="p-2 hover:bg-white/10 rounded-lg transition-colors">
            <ChevronLeft className="w-4 h-4 text-slate-400" />
          </button>
          <div className="text-sm font-bold text-white tracking-widest uppercase">
            {monthNames[currentDate.getMonth()]} {currentDate.getFullYear()}
          </div>
          <button onClick={nextMonth} className="p-2 hover:bg-white/10 rounded-lg transition-colors">
            <ChevronRight className="w-4 h-4 text-slate-400" />
          </button>
        </div>

        <div className="grid grid-cols-7 gap-2 mb-2 text-center text-xs font-bold text-slate-500">
          {dayNames.map(day => <div key={day}>{day}</div>)}
        </div>

        <div className="grid grid-cols-7 gap-2 text-center text-sm font-mono">
          {Array.from({ length: firstDay }).map((_, i) => (
            <div key={`empty-${i}`} className="py-2" />
          ))}
          {Array.from({ length: daysInMonth }).map((_, i) => {
            const day = i + 1;
            const isToday = 
              day === new Date().getDate() && 
              currentDate.getMonth() === new Date().getMonth() && 
              currentDate.getFullYear() === new Date().getFullYear();

            return (
              <div 
                key={day} 
                className={`py-2 rounded-lg transition-colors cursor-pointer ${
                  isToday 
                    ? 'bg-atc-cyan text-slate-900 font-bold shadow-[0_0_10px_rgba(6,182,212,0.3)]' 
                    : 'text-slate-300 hover:bg-white/5'
                }`}
              >
                {day}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
