import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { ATC_OS_REQUIREMENTS } from '../requirementsData';

export function DependencyMapView() {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current) return;

    const width = 800;
    const height = 600;

    // Clear previous
    d3.select(svgRef.current).selectAll('*').remove();

    const svg = d3.select(svgRef.current)
      .attr('viewBox', [-width / 2, -height / 2, width, height])
      .attr('width', '100%')
      .attr('height', '100%')
      .style('background', 'transparent');

    const nodes: any[] = [];
    const links: any[] = [];

    // Root node
    nodes.push({ id: 'ATC_OS', group: 0, radius: 16 });

    ATC_OS_REQUIREMENTS.forEach((cat, catIndex) => {
      nodes.push({ id: cat.title, group: 1, radius: 10 });
      links.push({ source: 'ATC_OS', target: cat.title, distance: 100 });

      cat.packages.forEach((pkg, pkgIndex) => {
        nodes.push({ id: pkg, group: 2, radius: 5 });
        links.push({ source: cat.title, target: pkg, distance: 50 });
      });
    });

    const simulation = d3.forceSimulation(nodes)
      .force("link", d3.forceLink(links).id((d: any) => d.id).distance((d: any) => d.distance))
      .force("charge", d3.forceManyBody().strength(-150))
      .force("center", d3.forceCenter(0, 0))
      .force("collide", d3.forceCollide().radius((d: any) => d.radius + 5));

    const link = svg.append("g")
      .attr("stroke", "rgba(255, 255, 255, 0.1)")
      .attr("stroke-width", 1.5)
      .selectAll("line")
      .data(links)
      .join("line");

    const nodeGroup = svg.append("g")
      .selectAll("g")
      .data(nodes)
      .join("g")
      .call(d3.drag()
        .on("start", dragstarted)
        .on("drag", dragged)
        .on("end", dragended) as any);

    nodeGroup.append("circle")
      .attr("r", d => d.radius)
      .attr("fill", d => d.group === 0 ? '#10b981' : d.group === 1 ? '#a855f7' : '#06b6d4')
      .attr('stroke', 'rgba(255,255,255,0.2)')
      .attr('stroke-width', 2);

    nodeGroup.append('text')
      .attr('dy', '0.31em')
      .attr('x', d => d.radius + 6)
      .text(d => d.id)
      .style('fill', '#94a3b8')
      .style('font-size', d => d.group === 0 ? '14px' : d.group === 1 ? '12px' : '10px')
      .style('font-family', 'monospace')
      .style('pointer-events', 'none')
      .style('text-shadow', '0 1px 3px rgba(0,0,0,0.8)');

    simulation.on("tick", () => {
      link
        .attr("x1", (d: any) => d.source.x)
        .attr("y1", (d: any) => d.source.y)
        .attr("x2", (d: any) => d.target.x)
        .attr("y2", (d: any) => d.target.y);

      nodeGroup
        .attr("transform", (d: any) => `translate(${d.x},${d.y})`);
    });

    function dragstarted(event: any, d: any) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      d.fx = d.x;
      d.fy = d.y;
    }

    function dragged(event: any, d: any) {
      d.fx = event.x;
      d.fy = event.y;
    }

    function dragended(event: any, d: any) {
      if (!event.active) simulation.alphaTarget(0);
      d.fx = null;
      d.fy = null;
    }

    return () => {
      simulation.stop();
    };

  }, []);

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="flex gap-4 p-4 border-b border-white/5 bg-black/20 shrink-0">
        <h2 className="text-xl font-semibold text-white">Dependency Map</h2>
      </div>
      <div className="flex-1 p-6 relative overflow-auto min-h-0">
        <div className="absolute inset-0 flex items-center justify-center p-8">
           <svg ref={svgRef} className="w-full h-full" />
        </div>
      </div>
    </div>
  );
}
