import React, { useState, useEffect, useRef } from 'react';
import { Cpu, Activity, Thermometer } from 'lucide-react';
import * as d3 from 'd3';

export function GpuPerformanceWidget() {
  const [gpuStats, setGpuStats] = useState({
    utilization: 0,
    temperature: 0,
    vram: 0
  });

  const [history, setHistory] = useState<number[]>(Array(20).fill(0));
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    const interval = setInterval(() => {
      const util = Math.floor(Math.random() * 40) + 10;
      setGpuStats({
        utilization: util,
        temperature: Math.floor(Math.random() * 20) + 40,
        vram: Math.floor(Math.random() * 2000) + 4000
      });
      setHistory(prev => {
        const newHist = [...prev.slice(1), util];
        return newHist;
      });
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (!svgRef.current) return;
    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove();

    const width = 60;
    const height = 24;
    const margin = { top: 2, right: 0, bottom: 2, left: 0 };

    const innerWidth = width - margin.left - margin.right;
    const innerHeight = height - margin.top - margin.bottom;

    const x = d3.scaleLinear()
      .domain([0, history.length - 1])
      .range([0, innerWidth]);

    const y = d3.scaleLinear()
      .domain([0, 100])
      .range([innerHeight, 0]);

    const line = d3.line<number>()
      .x((_, i) => x(i))
      .y(d => y(d))
      .curve(d3.curveMonotoneX);

    const g = svg.append('g').attr('transform', `translate(${margin.left},${margin.top})`);

    g.append('path')
      .datum(history)
      .attr('fill', 'none')
      .attr('stroke', '#10b981')
      .attr('stroke-width', 1.5)
      .attr('d', line);

    // subtle area under line
    const area = d3.area<number>()
      .x((_, i) => x(i))
      .y0(innerHeight)
      .y1(d => y(d))
      .curve(d3.curveMonotoneX);

    g.append('path')
      .datum(history)
      .attr('fill', 'rgba(16, 185, 129, 0.2)')
      .attr('d', area);

  }, [history]);

  return (
    <div className="flex items-center gap-3 px-3 py-1.5 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-slate-300 relative group cursor-pointer">
      <div className="flex items-center gap-2">
        <Cpu className="w-3.5 h-3.5 text-emerald-400 group-hover:text-emerald-300 transition-colors" />
        <span className="text-[10px] font-mono font-bold text-emerald-400 w-8">{gpuStats.utilization}%</span>
      </div>
      
      <svg ref={svgRef} width="60" height="24" className="overflow-visible" />
      
      {/* Tooltip */}
      <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-3 bg-[#090b14]/95 backdrop-blur-md border border-white/10 rounded-lg shadow-xl p-3 opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-[10000] w-48 pointer-events-none">
        <div className="flex items-center gap-2 mb-2 pb-2 border-b border-white/10">
          <Cpu className="w-4 h-4 text-emerald-400" />
          <span className="text-xs font-bold uppercase tracking-widest text-slate-300">GPU Status</span>
        </div>
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Activity className="w-3 h-3 text-emerald-400" />
              <span className="text-[10px] font-mono text-slate-400">Core</span>
            </div>
            <span className="text-xs font-mono font-bold text-emerald-400">{gpuStats.utilization}%</span>
          </div>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Thermometer className="w-3 h-3 text-amber-400" />
              <span className="text-[10px] font-mono text-slate-400">Temp</span>
            </div>
            <span className="text-xs font-mono font-bold text-amber-400">{gpuStats.temperature}°C</span>
          </div>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Cpu className="w-3 h-3 text-cyan-400" />
              <span className="text-[10px] font-mono text-slate-400">VRAM</span>
            </div>
            <span className="text-xs font-mono font-bold text-cyan-400">{gpuStats.vram} MB</span>
          </div>
        </div>
      </div>
    </div>
  );
}
