import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';

interface Commit {
  id: string;
  message: string;
  author: string;
  date: string;
  branch: string;
  parents: string[];
}

const MOCK_COMMITS: Commit[] = [
  { id: '1a2b3c4', message: 'Initial commit', author: 'Alice', date: '2026-06-10T10:00:00Z', branch: 'main', parents: [] },
  { id: '5d6e7f8', message: 'Add core architecture', author: 'Bob', date: '2026-06-11T12:00:00Z', branch: 'main', parents: ['1a2b3c4'] },
  { id: '9g0h1i2', message: 'Implement networking layer', author: 'Charlie', date: '2026-06-12T14:30:00Z', branch: 'feature/networking', parents: ['5d6e7f8'] },
  { id: '3j4k5l6', message: 'Update consensus docs', author: 'Alice', date: '2026-06-13T09:15:00Z', branch: 'main', parents: ['5d6e7f8'] },
  { id: '7m8n9o0', message: 'Merge networking feature', author: 'Bob', date: '2026-06-14T16:45:00Z', branch: 'main', parents: ['3j4k5l6', '9g0h1i2'] },
  { id: '1p2q3r4', message: 'Fix sandbox issue', author: 'Charlie', date: '2026-06-15T11:20:00Z', branch: 'main', parents: ['7m8n9o0'] },
];

export function GitGraphVisualization() {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!containerRef.current) return;

    // Clear previous drawing
    d3.select(containerRef.current).selectAll('*').remove();

    const width = containerRef.current.clientWidth;
    const height = 400;
    const padding = 40;

    const svg = d3.select(containerRef.current)
      .append('svg')
      .attr('width', '100%')
      .attr('height', height)
      .attr('viewBox', `0 0 ${width} ${height}`);

    // Simple layout logic for branches
    const branchY: Record<string, number> = {
      'main': height / 2,
      'feature/networking': height / 2 - 80,
    };

    const xScale = d3.scaleTime()
      .domain(d3.extent(MOCK_COMMITS, d => new Date(d.date)) as [Date, Date])
      .range([padding, width - padding]);

    // Plot links
    const linkGen = d3.linkHorizontal()
      .x(d => (d as any)[0])
      .y(d => (d as any)[1]);

    const links: { source: [number, number], target: [number, number] }[] = [];

    MOCK_COMMITS.forEach(commit => {
      const targetDate = new Date(commit.date);
      const targetX = xScale(targetDate);
      const targetY = branchY[commit.branch] || height / 2;

      commit.parents.forEach(parentId => {
        const parent = MOCK_COMMITS.find(c => c.id === parentId);
        if (parent) {
          const sourceDate = new Date(parent.date);
          const sourceX = xScale(sourceDate);
          const sourceY = branchY[parent.branch] || height / 2;
          links.push({
            source: [sourceX, sourceY],
            target: [targetX, targetY]
          });
        }
      });
    });

    svg.selectAll('.link')
      .data(links)
      .enter()
      .append('path')
      .attr('class', 'link')
      .attr('d', d => {
        // Create an elegant curved path
        const curve = d3.linkHorizontal()
          .x(pt => (pt as any)[0])
          .y(pt => (pt as any)[1]);
        return curve({ source: d.source, target: d.target });
      })
      .attr('fill', 'none')
      .attr('stroke', '#334155')
      .attr('stroke-width', 2);

    // Plot nodes
    const nodes = svg.selectAll('.node')
      .data(MOCK_COMMITS)
      .enter()
      .append('g')
      .attr('class', 'node')
      .attr('transform', d => `translate(${xScale(new Date(d.date))},${branchY[d.branch] || height / 2})`);

    nodes.append('circle')
      .attr('r', 8)
      .attr('fill', d => d.branch === 'main' ? '#00d1ff' : '#ec4899')
      .attr('stroke', '#0f172a')
      .attr('stroke-width', 3);

    // Add labels
    nodes.append('text')
      .attr('dy', 25)
      .attr('text-anchor', 'middle')
      .attr('fill', '#94a3b8')
      .attr('font-size', '10px')
      .attr('font-family', 'monospace')
      .text(d => d.id.substring(0, 7));

    // Add tooltips
    nodes.append('title')
      .text(d => `${d.message}\nBy: ${d.author}\nOn: ${new Date(d.date).toLocaleString()}`);

  }, []);

  return (
    <div className="w-full h-full min-h-[400px] bg-[#090b14]/50 border border-white/5 rounded-xl overflow-hidden relative">
      <div className="absolute top-4 left-4 flex gap-4">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-[#00d1ff]" />
          <span className="text-xs font-mono text-slate-400">main</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-pink-500" />
          <span className="text-xs font-mono text-slate-400">feature</span>
        </div>
      </div>
      <div ref={containerRef} className="w-full h-full" />
    </div>
  );
}
