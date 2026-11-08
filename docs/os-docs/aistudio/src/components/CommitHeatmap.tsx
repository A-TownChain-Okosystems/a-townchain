import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';

export function CommitHeatmap({ repoId }: { repoId: string }) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!containerRef.current) return;

    // Deterministic random based on repoId
    let seed = repoId.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
    const rand = () => {
      seed = (seed * 9301 + 49297) % 233280;
      return seed / 233280;
    };

    const now = new Date();
    const data: { date: Date; count: number }[] = [];
    for (let i = 364; i >= 0; i--) {
      const date = new Date(now);
      date.setDate(date.getDate() - i);
      data.push({
        date,
        count: rand() > 0.7 ? Math.floor(rand() * 10) : 0
      });
    }

    const margin = { top: 20, right: 20, bottom: 20, left: 40 };
    const width = containerRef.current.clientWidth - margin.left - margin.right;
    const height = 150 - margin.top - margin.bottom;

    // Clear previous SVG
    d3.select(containerRef.current).selectAll('*').remove();

    const svg = d3.select(containerRef.current)
      .append('svg')
      .attr('width', width + margin.left + margin.right)
      .attr('height', height + margin.top + margin.bottom)
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    const cellSize = Math.min(14, width / 53);
    const weeks = 53;
    const days = 7;

    const timeWeek = d3.timeSunday;
    
    // Create color scale
    const colorScale = d3.scaleThreshold<number, string>()
      .domain([1, 3, 6, 9])
      .range(['#1e293b', '#0e4429', '#006d32', '#26a641', '#39d353']);

    // Tooltip
    const tooltip = d3.select(containerRef.current)
      .append('div')
      .style('opacity', 0)
      .attr('class', 'absolute bg-[#00101c]/95 border border-atc-cyan/30 text-white p-2 rounded-lg pointer-events-none text-xs transform -translate-x-1/2 -translate-y-full mb-2 backdrop-blur-xl shadow-2xl z-10 transition-opacity whitespace-nowrap');

    const yearData = d3.group(data, d => timeWeek.count(d3.timeYear(d.date), d.date));

    svg.append('g')
      .selectAll('rect')
      .data(data)
      .enter()
      .append('rect')
      .attr('width', cellSize - 1)
      .attr('height', cellSize - 1)
      .attr('x', d => timeWeek.count(d3.timeYear(d.date), d.date) * cellSize)
      .attr('y', d => d.date.getDay() * cellSize)
      .attr('fill', d => d.count === 0 ? '#1e293b' : colorScale(d.count))
      .attr('rx', 2)
      .attr('ry', 2)
      .on('mouseover', function(event, d) {
        d3.select(this).attr('stroke', '#06b6d4').attr('stroke-width', 1);
        tooltip.transition().duration(200).style('opacity', 1);
        tooltip.html(`<div class="font-bold text-atc-cyan mb-1">${d.date.toLocaleDateString()}</div><div>${d.count} commits</div>`)
          .style('left', (event.pageX) + 'px')
          .style('top', (event.pageY - 10) + 'px');
      })
      .on('mouseout', function(event, d) {
        d3.select(this).attr('stroke', 'none');
        tooltip.transition().duration(500).style('opacity', 0);
      });

    // Month labels
    const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    svg.selectAll('.month-label')
      .data(monthNames)
      .enter()
      .append('text')
      .attr('class', 'month-label')
      .attr('x', (d, i) => (i * (53 / 12)) * cellSize)
      .attr('y', -5)
      .text(d => d)
      .attr('font-size', '10px')
      .attr('fill', '#64748b')
      .attr('font-family', 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace');

  }, [repoId]);

  return (
    <div className="w-full mt-6 pt-4 border-t border-white/10">
      <h4 className="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider mb-4 flex items-center gap-2">
        <svg className="w-4 h-4 text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
        Activity Graph
      </h4>
      <div className="relative w-full overflow-x-auto custom-scrollbar" ref={containerRef} />
    </div>
  );
}
