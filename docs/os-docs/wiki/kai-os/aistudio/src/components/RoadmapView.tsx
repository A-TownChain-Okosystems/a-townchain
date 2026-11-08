import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { Activity, ServerCrash, CheckCircle2 } from 'lucide-react';

export function RoadmapView() {
  const d3Container = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!d3Container.current) return;

    // Clear any existing SVG
    d3.select(d3Container.current).selectAll('*').remove();

    // Simulated Sync History Data
    const generateData = () => {
      const data = [];
      const now = new Date();
      for (let i = 60; i >= 0; i--) {
        data.push({
          date: new Date(now.getTime() - i * 24 * 60 * 60 * 1000), // last 60 days
          syncRate: Math.max(50, 100 - Math.random() * 30 + (i < 10 ? 10 : 0)),
          failureRate: Math.max(0, Math.random() * 20 - (i < 10 ? 10 : 0))
        });
      }
      return data;
    };

    const data = generateData();

    // Dimensions
    const margin = { top: 20, right: 30, bottom: 30, left: 40 };
    const width = d3Container.current.clientWidth - margin.left - margin.right;
    const height = 400 - margin.top - margin.bottom;

    const svg = d3.select(d3Container.current)
      .append('svg')
      .attr('width', width + margin.left + margin.right)
      .attr('height', height + margin.top + margin.bottom)
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    // X axis => Date
    const x = d3.scaleTime()
      .domain(d3.extent(data, d => d.date) as [Date, Date])
      .range([0, width]);

    svg.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x).ticks(5))
      .attr('color', '#64748b') // slate-500
      .selectAll("text")
      .style('font-family', 'monospace')
      .style('font-size', '10px');

    // Y axis => Percentage
    const y = d3.scaleLinear()
      .domain([0, 100])
      .range([height, 0]);

    svg.append('g')
      .call(d3.axisLeft(y).ticks(5))
      .attr('color', '#64748b')
      .selectAll("text")
      .style('font-family', 'monospace')
      .style('font-size', '10px');

    // Add Y axis grid lines
    svg.append('g')
      .attr('class', 'grid')
      .call(d3.axisLeft(y).tickSize(-width).tickFormat(() => ''))
      .attr('color', 'rgba(255,255,255,0.05)')
      .select('.domain').remove();

    // Line for Sync Rate
    const syncLine = d3.line<any>()
      .x(d => x(d.date))
      .y(d => y(d.syncRate))
      .curve(d3.curveMonotoneX);

    svg.append('path')
      .datum(data)
      .attr('fill', 'none')
      .attr('stroke', '#10b981') // emerald-500
      .attr('stroke-width', 2)
      .attr('d', syncLine);

    // Area for Sync Rate
    const syncArea = d3.area<any>()
      .x(d => x(d.date))
      .y0(height)
      .y1(d => y(d.syncRate))
      .curve(d3.curveMonotoneX);

    svg.append('path')
      .datum(data)
      .attr('fill', 'url(#sync-gradient)')
      .attr('d', syncArea)
      .attr('opacity', 0.2);

    // Line for Failure Rate
    const failLine = d3.line<any>()
      .x(d => x(d.date))
      .y(d => y(d.failureRate))
      .curve(d3.curveMonotoneX);

    svg.append('path')
      .datum(data)
      .attr('fill', 'none')
      .attr('stroke', '#ef4444') // red-500
      .attr('stroke-width', 2)
      .attr('d', failLine);

    // Area for Failure Rate
    const failArea = d3.area<any>()
      .x(d => x(d.date))
      .y0(height)
      .y1(d => y(d.failureRate))
      .curve(d3.curveMonotoneX);

    svg.append('path')
      .datum(data)
      .attr('fill', 'url(#fail-gradient)')
      .attr('d', failArea)
      .attr('opacity', 0.2);

    // Gradients
    const defs = svg.append('defs');

    // Sync Gradient
    const syncGrad = defs.append('linearGradient')
      .attr('id', 'sync-gradient')
      .attr('x1', '0%').attr('y1', '0%')
      .attr('x2', '0%').attr('y2', '100%');
    syncGrad.append('stop').attr('offset', '0%').style('stop-color', '#10b981').style('stop-opacity', 1);
    syncGrad.append('stop').attr('offset', '100%').style('stop-color', '#10b981').style('stop-opacity', 0);

    // Fail Gradient
    const failGrad = defs.append('linearGradient')
      .attr('id', 'fail-gradient')
      .attr('x1', '0%').attr('y1', '0%')
      .attr('x2', '0%').attr('y2', '100%');
    failGrad.append('stop').attr('offset', '0%').style('stop-color', '#ef4444').style('stop-opacity', 1);
    failGrad.append('stop').attr('offset', '100%').style('stop-color', '#ef4444').style('stop-opacity', 0);

  }, []);

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans px-8">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
          <Activity className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Roadmap & Sync Dashboard
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Live monitoring von Roadmap-Synchronisation, Netzwerkausfällen und automatisierten Rollout-Historien.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
         <div className="bg-[#090b14] border border-white/5 rounded-2xl p-6 flex flex-col gap-2 shadow-lg">
            <h4 className="text-sm text-slate-400 flex items-center gap-2.5 font-mono"><CheckCircle2 className="w-4 h-4 text-emerald-500" /> Durchschnittl. Sync Rate</h4>
            <span className="text-4xl font-bold text-emerald-400 font-mono tracking-tight">92.4%</span>
         </div>
         <div className="bg-[#090b14] border border-white/5 rounded-2xl p-6 flex flex-col gap-2 shadow-lg">
            <h4 className="text-sm text-slate-400 flex items-center gap-2.5 font-mono"><ServerCrash className="w-4 h-4 text-red-500" /> Network Failures (30d)</h4>
            <span className="text-4xl font-bold text-red-400 font-mono tracking-tight">1.2%</span>
         </div>
      </div>

      <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col shadow-xl">
        <div className="flex items-center justify-between mb-8">
          <div>
            <h3 className="text-xl font-bold text-slate-200">Sync History Trends</h3>
            <p className="text-sm text-slate-500">D3.js Visualization: Sync Success vs Failures</p>
          </div>
          <div className="flex gap-4">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-emerald-500"></div>
              <span className="text-xs text-slate-400 font-mono">Sync Rate</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-red-500"></div>
              <span className="text-xs text-slate-400 font-mono">Failure Rate</span>
            </div>
          </div>
        </div>
        
        <div className="w-full h-[400px]" ref={d3Container}></div>
      </div>
    </div>
  );
}
