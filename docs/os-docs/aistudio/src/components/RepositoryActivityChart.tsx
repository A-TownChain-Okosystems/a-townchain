import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';

interface Props {
  repoId: string;
  language?: 'EN' | 'DE';
}

const translations = {
  EN: {
    title: '30-Day Contribution Frequency',
    descriptionPrefix: 'Showing commit volume for',
    descriptionSuffix: 'over the last month.',
    commits: 'commits'
  },
  DE: {
    title: '30-Tage Beitragsfrequenz',
    descriptionPrefix: 'Zeigt das Commit-Volumen für',
    descriptionSuffix: 'im letzten Monat.',
    commits: 'Commits'
  }
};

export function RepositoryActivityChart({ repoId, language = 'DE' }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const t = translations[language];

  useEffect(() => {
    if (!containerRef.current) return;

    // Generate pseudo-random realistic looking data
    let seed = repoId.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
    const rand = () => {
      seed = (seed * 9301 + 49297) % 233280;
      return seed / 233280;
    };

    const data = Array.from({ length: 30 }, (_, i) => {
      const d = new Date();
      d.setDate(d.getDate() - (29 - i));
      return {
        date: d,
        commits: Math.floor(rand() * 15 * (rand() > 0.3 ? 1 : 0))
      };
    });

    const margin = { top: 20, right: 20, bottom: 30, left: 40 };
    const width = containerRef.current.clientWidth - margin.left - margin.right;
    const height = 300 - margin.top - margin.bottom;

    // Clear previous SVG
    d3.select(containerRef.current).selectAll('*').remove();

    const svg = d3.select(containerRef.current)
      .append('svg')
      .attr('width', width + margin.left + margin.right)
      .attr('height', height + margin.top + margin.bottom)
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    const x = d3.scaleBand()
      .range([0, width])
      .padding(0.2)
      .domain(data.map(d => d.date.toLocaleDateString(navigator.language, { month: 'short', day: 'numeric' })));

    svg.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x).tickValues(x.domain().filter((_, i) => i % 5 === 0)))
      .attr('color', '#64748b') // text-slate-500
      .selectAll('text')
      .attr('transform', 'translate(-10,0)rotate(-45)')
      .style('text-anchor', 'end');

    const maxVal = d3.max(data, d => d.commits) || 10;
    const y = d3.scaleLinear()
      .domain([0, maxVal + 5])
      .range([height, 0]);

    svg.append('g')
      .call(d3.axisLeft(y).ticks(5))
      .attr('color', '#64748b'); // text-slate-500

    // Add Gridlines
    svg.append('g')
      .attr('class', 'grid')
      .call(d3.axisLeft(y).tickSize(-width).tickFormat(() => '').ticks(5))
      .style('stroke-opacity', 0.1)
      .attr('color', '#ffffff');

    // Tooltip setup
    const tooltip = d3.select(containerRef.current)
      .append('div')
      .style('opacity', 0)
      .attr('class', 'absolute bg-[#00101c]/95 border border-atc-cyan/30 text-white p-2 rounded-lg pointer-events-none text-xs transform -translate-x-1/2 -translate-y-full mb-2 backdrop-blur-xl shadow-2xl z-10 transition-opacity whitespace-nowrap')

    // Append bars
    svg.selectAll('.bar')
      .data(data)
      .enter()
      .append('rect')
      .attr('class', 'bar')
      .attr('x', d => x(d.date.toLocaleDateString(navigator.language, { month: 'short', day: 'numeric' })) || 0)
      .attr('width', x.bandwidth())
      .attr('y', height)
      .attr('height', 0)
      .attr('fill', d => d.commits === 0 ? '#1e293b' : '#14b8a6') // tailwind teal-500
      .attr('rx', 2)
      .on('mouseover', function(event, d) {
        d3.select(this)
          .attr('opacity', 0.8)
          .attr('fill', '#06b6d4'); // tailwind cyan-500
        
        tooltip.transition().duration(200).style('opacity', 1);
        tooltip.html(`<div class="font-bold text-atc-cyan mb-1">${d.date.toLocaleDateString()}</div><div>${d.commits} ${t.commits}</div>`)
          .style('left', (event.pageX) + 'px')
          .style('top', (event.pageY - 10) + 'px');
      })
      .on('mouseout', function(event, d) {
        d3.select(this)
          .attr('opacity', 1)
          .attr('fill', d.commits === 0 ? '#1e293b' : '#14b8a6');
        tooltip.transition().duration(500).style('opacity', 0);
      });

    // Animation transition
    svg.selectAll('.bar')
      .transition()
      .duration(800)
      .attr('y', (d: any) => y(d.commits))
      .attr('height', (d: any) => height - y(d.commits))
      .delay((_: any, i: number) => i * 20);

  }, [repoId, language, t]);

  return (
    <div className="w-full bg-[#090b14]/80 border border-white/5 rounded-2xl p-6 backdrop-blur-md relative h-full">
      <h3 className="text-xl font-bold text-white tracking-tight flex items-center gap-2 mb-2 font-mono">
        <svg className="w-5 h-5 text-teal-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path></svg>
        {t.title}
      </h3>
      <p className="text-sm text-slate-400 mb-6">{t.descriptionPrefix} <span className="text-teal-400 font-mono font-bold">{repoId}</span> {t.descriptionSuffix}</p>
      <div className="relative w-full h-[300px]" ref={containerRef} />
    </div>
  );
}
