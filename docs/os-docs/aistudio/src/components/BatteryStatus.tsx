import React, { useState, useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { Battery, BatteryWarning } from 'lucide-react';

export function BatteryStatus() {
  const [level, setLevel] = useState(100);
  const [consumption, setConsumption] = useState<number[]>(Array(20).fill(50));
  const [contextMenu, setContextMenu] = useState<{ x: number, y: number } | null>(null);
  const [powerMode, setPowerMode] = useState<'high-performance'|'balanced'|'eco'>(() => {
    return (localStorage.getItem('atc_power_mode') as any) || 'balanced';
  });
  const [isPowerSaver, setIsPowerSaver] = useState(() => localStorage.getItem('atc_power_mode') === 'eco'); // Also controlled by eco mode
  const [isStressTest, setIsStressTest] = useState(false);
  const svgRef = useRef<SVGSVGElement>(null);
  const tooltipSvgRef = useRef<SVGSVGElement>(null);

  const [history24h] = useState(() => {
     return Array.from({ length: 24 }).map((_, i) => ({
       hour: 24 - i,
       draw: 30 + Math.sin(i / 3) * 20 + Math.random() * 15
     })).reverse();
  });

  useEffect(() => {
    const handleContextMenuClose = () => setContextMenu(null);
    document.addEventListener('click', handleContextMenuClose);
    return () => document.removeEventListener('click', handleContextMenuClose);
  }, []);

  useEffect(() => {
    const handleStressTest = (e: any) => setIsStressTest(e.detail);
    const handlePowerMode = (e: any) => {
      setPowerMode(e.detail);
      if (e.detail === 'eco') setIsPowerSaver(true);
      else setIsPowerSaver(false);
    };
    window.addEventListener('ATC_SET_STRESS_TEST', handleStressTest);
    window.addEventListener('ATC_SET_POWER_MODE', handlePowerMode);
    return () => {
      window.removeEventListener('ATC_SET_STRESS_TEST', handleStressTest);
      window.removeEventListener('ATC_SET_POWER_MODE', handlePowerMode);
    };
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      let currentDraw = 40 + Math.random() * 40; // balanced
      
      if (powerMode === 'high-performance') currentDraw = 70 + Math.random() * 60;
      if (isPowerSaver || powerMode === 'eco') currentDraw = 10 + Math.random() * 10;
      if (isStressTest) currentDraw = 150 + Math.random() * 50;

      setConsumption(prev => [...prev.slice(1), currentDraw]);
      
      setLevel(prev => {
        let dropChance = 0.2; // balanced
        if (powerMode === 'high-performance') dropChance = 0.5;
        if (isPowerSaver || powerMode === 'eco') dropChance = 0.05;
        if (isStressTest) dropChance = 0.8;
        
        let dropAmount = 0;
        if (Math.random() < dropChance) dropAmount = isStressTest ? 2 : 1;
        
        return (prev - dropAmount <= 0) ? 100 : prev - dropAmount;
      });
    }, 2000); 

    return () => clearInterval(interval);
  }, [isPowerSaver, isStressTest, powerMode]);

  const [hasWarnedLow, setHasWarnedLow] = useState(false);

  useEffect(() => {
    if (level < 15 && !hasWarnedLow) {
      window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Low Battery: Virtual power dropped below 15%' }));
      window.dispatchEvent(new CustomEvent('ATC_LOW_POWER_MODE', { detail: true }));
      setHasWarnedLow(true);
    } else if (level >= 15 && hasWarnedLow) {
      window.dispatchEvent(new CustomEvent('ATC_LOW_POWER_MODE', { detail: false }));
      setHasWarnedLow(false);
    }
  }, [level, hasWarnedLow]);

  useEffect(() => {
    if (!svgRef.current || consumption.length === 0) return;
    
    const svg = d3.select(svgRef.current);
    const width = 45;
    const height = 16;
    
    svg.selectAll("*").remove();
    
    const xScale = d3.scaleLinear()
      .domain([0, consumption.length - 1])
      .range([0, width]);
      
    const yScale = d3.scaleLinear()
      .domain([0, 100])
      .range([height, 0]);
    
    const line = d3.line<number>()
      .x((_, i) => xScale(i))
      .y(d => yScale(d))
      .curve(d3.curveMonotoneX);

    svg.append("path")
      .datum(consumption)
      .attr("fill", "none")
      .attr("stroke", level < 15 ? "#ef4444" : level < 20 ? "#f59e0b" : "#34d399")
      .attr("stroke-width", 1.5)
      .attr("d", line);
  }, [consumption, level]);

  useEffect(() => {
    if (!tooltipSvgRef.current) return;
    
    const svg = d3.select(tooltipSvgRef.current);
    const width = 200;
    const height = 80;
    const margin = { top: 10, right: 10, bottom: 20, left: 25 };
    
    svg.selectAll("*").remove();

    const innerWidth = width - margin.left - margin.right;
    const innerHeight = height - margin.top - margin.bottom;

    const g = svg.append("g")
      .attr("transform", `translate(${margin.left},${margin.top})`);
      
    const xScale = d3.scaleLinear()
      .domain([0, 23])
      .range([0, innerWidth]);
      
    const yScale = d3.scaleLinear()
      .domain([0, Math.max(100, d3.max(history24h, d => d.draw) || 100)])
      .range([innerHeight, 0]);
      
    const line = d3.line<typeof history24h[0]>()
      .x((_, i) => xScale(i))
      .y(d => yScale(d.draw))
      .curve(d3.curveMonotoneX);

    const area = d3.area<typeof history24h[0]>()
      .x((_, i) => xScale(i))
      .y0(innerHeight)
      .y1(d => yScale(d.draw))
      .curve(d3.curveMonotoneX);

    const defs = svg.append("defs");
    const gradient = defs.append("linearGradient")
      .attr("id", "power-gradient")
      .attr("x1", "0%").attr("y1", "0%")
      .attr("x2", "0%").attr("y2", "100%");
    gradient.append("stop").attr("offset", "0%").attr("stop-color", "#6366f1").attr("stop-opacity", 0.4);
    gradient.append("stop").attr("offset", "100%").attr("stop-color", "#6366f1").attr("stop-opacity", 0);

    g.append("path")
      .datum(history24h)
      .attr("fill", "url(#power-gradient)")
      .attr("d", area);

    g.append("path")
      .datum(history24h)
      .attr("fill", "none")
      .attr("stroke", "#6366f1")
      .attr("stroke-width", 1.5)
      .attr("d", line);
      
    const xAxis = d3.axisBottom(xScale).ticks(4).tickFormat((d) => `-${24 - Number(d)}h`).tickSize(3);
    const yAxis = d3.axisLeft(yScale).ticks(3).tickSize(3);
    
    g.append("g")
      .attr("transform", `translate(0,${innerHeight})`)
      .call(xAxis)
      .attr("class", "text-[8px] text-slate-500 font-mono")
      .select(".domain").remove();

    g.append("g")
      .call(yAxis)
      .attr("class", "text-[8px] text-slate-500 font-mono")
      .select(".domain").remove();
      
  }, [history24h]);

  return (
    <div 
      className={`flex items-center gap-1.5 px-2 py-1 hover:bg-white/10 rounded-md transition-colors cursor-pointer group relative ${level < 15 ? 'text-red-400 animate-pulse bg-red-900/20' : 'text-slate-300'}`} 
      title={`Virtual Power: ${level}%`}
      onContextMenu={(e) => {
        e.preventDefault();
        setContextMenu({ x: e.clientX, y: e.clientY });
      }}
    >
      {level < 15 ? <BatteryWarning className="w-4 h-4" /> : <Battery className="w-4 h-4" />}
      <svg ref={svgRef} width="45" height="16" className="opacity-90 hidden sm:block" />
      <span className="text-xs font-mono">{level}%</span>
      
      {/* Tooltip */}
      <div className="absolute bottom-full right-0 mb-2 p-3 bg-[#090b14]/95 backdrop-blur-xl border border-white/10 rounded-xl shadow-2xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-40 pointer-events-none group-hover:pointer-events-auto cursor-default">
        <div className="text-sm font-semibold text-white mb-1 flex items-center justify-between gap-4">
           Virtual Power Monitor 
           {level < 15 && <span className="px-1.5 py-0.5 rounded text-[9px] bg-red-500/20 text-red-400 uppercase tracking-wider">Low Power Mode</span>}
           {isPowerSaver && <span className="px-1.5 py-0.5 rounded text-[9px] bg-indigo-500/20 text-indigo-400 uppercase tracking-wider">Power Saver</span>}
           {isStressTest && <span className="px-1.5 py-0.5 rounded text-[9px] bg-orange-500/20 text-orange-400 uppercase tracking-wider">Stress Test</span>}
        </div>
        <div className="flex items-center gap-4 mb-2">
          <div className="text-[10px] text-slate-400 font-mono">Current Draw: <span className="text-indigo-300">{Math.round(consumption[consumption.length - 1] || 0)}W</span></div>
          <div className="text-[10px] text-slate-400 font-mono">Battery: <span className={level < 15 ? 'text-red-400 animate-pulse' : 'text-emerald-400'}>{level}%</span></div>
        </div>
        
        <div className="mt-3 pt-2 border-t border-white/5">
          <div className="text-[10px] text-slate-500 font-mono mb-1 uppercase tracking-wider">24h Power History</div>
          <svg ref={tooltipSvgRef} width="200" height="80" className="opacity-90 pointer-events-none" />
        </div>

        {level < 15 && (
          <div className="mt-2 pt-2 border-t border-red-500/20">
            <button 
              onClick={(e) => {
                e.stopPropagation();
                window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'atc_hard_settings' }));
              }}
              className="w-full py-1.5 px-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 text-[10px] font-mono rounded flex items-center justify-between transition-colors outline-none focus:ring-1 focus:ring-red-500/50"
            >
              <span>Enable Power Saving</span>
              <span className="text-white opacity-60">&rarr;</span>
            </button>
          </div>
        )}
      </div>

      {contextMenu && (
        <div 
          className="fixed z-50 bg-[#0a0d16]/95 backdrop-blur-xl border border-white/10 rounded-xl shadow-2xl py-2 min-w-[200px]"
          style={{ top: contextMenu.y - 120, left: contextMenu.x }}
          onClick={(e) => e.stopPropagation()}
        >
          <button 
            className="w-full text-left px-4 py-2 text-sm text-slate-300 hover:text-white hover:bg-indigo-500/20 transition-colors flex justify-between items-center"
            onClick={() => { 
                const newMode = (powerMode === 'eco' || isPowerSaver) ? 'balanced' : 'eco';
                window.dispatchEvent(new CustomEvent('ATC_SET_POWER_MODE', { detail: newMode }));
                setPowerMode(newMode);
                setIsPowerSaver(newMode === 'eco');
                localStorage.setItem('atc_power_mode', newMode);
                setContextMenu(null); 
            }}
          >
            <span>Toggle Power Saver Mode</span>
            {isPowerSaver && <span className="w-2 h-2 rounded-full bg-emerald-500" />}
          </button>
          <div className="w-full h-px bg-white/10 my-1" />
          <button 
            className="w-full text-left px-4 py-2 text-sm text-slate-300 hover:text-white hover:bg-indigo-500/20 transition-colors"
            onClick={() => { 
                window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'atc_hard_settings' }));
                setTimeout(() => {
                  document.getElementById('power-management-section')?.scrollIntoView({ behavior: 'smooth' });
                }, 200);
                setContextMenu(null);
            }}
          >
            Power Management Options
          </button>
        </div>
      )}
    </div>
  );
}
