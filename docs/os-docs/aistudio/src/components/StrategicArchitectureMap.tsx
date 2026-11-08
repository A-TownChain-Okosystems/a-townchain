import React, { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';
import { LAYERS } from '../data';

const STRATEGIC_LAYERS = LAYERS;

export function StrategicArchitectureMap({ searchQuery = "" }: { searchQuery?: string }) {
  const svgRef = useRef<SVGSVGElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [tooltip, setTooltip] = useState<{ visible: boolean; x: number; y: number; title: string; content: string[] }>({ visible: false, x: 0, y: 0, title: '', content: [] });

  useEffect(() => {
    if (!svgRef.current) return;

    const width = 900;
    const height = 700;

    d3.select(svgRef.current).selectAll("*").remove();

    const svg = d3.select(svgRef.current)
      .attr("viewBox", [0, 0, width, height] as any)
      .style("background", "transparent");

    // Def for inner glow or gradient
    const defs = svg.append("defs");
    
    const filter = defs.append("filter").attr("id", "glow-arch");
    filter.append("feGaussianBlur").attr("stdDeviation", "2.5").attr("result", "coloredBlur");
    const feMerge = filter.append("feMerge");
    feMerge.append("feMergeNode").attr("in", "coloredBlur");
    feMerge.append("feMergeNode").attr("in", "SourceGraphic");

    // Prepare node and link arrays
    const nodes: any[] = [];
    const links: any[] = [];

    const rootId = "A-TownChain System";
    nodes.push({ id: rootId, name: rootId, type: 'root', group: 0, radius: 25 });

    STRATEGIC_LAYERS.forEach((layer, i) => {
      const layerId = layer.name;
      nodes.push({ id: layerId, name: layerId, type: 'layer', group: i + 1, radius: 15 });
      links.push({ source: rootId, target: layerId, distance: 150 });

      layer.subLayers.forEach(comp => {
        const compId = `${layer.name}-${comp}`;
        nodes.push({ id: compId, name: comp, type: 'component', group: i + 1, radius: 8, activity: Math.random() });
        links.push({ source: layerId, target: compId, distance: 60 });
      });
    });

    const simulation = d3.forceSimulation(nodes as any)
      .force("link", d3.forceLink(links).id((d: any) => d.id).distance((d: any) => d.distance))
      .force("charge", d3.forceManyBody().strength((d: any) => d.type === 'root' ? -500 : d.type === 'layer' ? -300 : -100))
      .force("center", d3.forceCenter(width / 2, height / 2))
      .force("collide", d3.forceCollide().radius((d: any) => d.radius + 15));

    const linkGroup = svg.append("g")
      .attr("stroke", "#334155")
      .attr("stroke-opacity", 0.4)
      .selectAll("line")
      .data(links)
      .join("line")
      .attr("stroke-width", (d: any) => d.source.type === 'root' ? 2 : 1);

    // active pulses
    const pulseLinks = svg.append("g")
      .selectAll("circle")
      .data(links.filter((d: any) => d.target.type === 'component'))
      .join("circle")
      .attr("r", 2)
      .attr("fill", "#06b6d4")
      .style("filter", "url(#glow-arch)")
      .style("opacity", 0);

    const nodeGroup = svg.append("g")
      .attr("class", "node-group")
      .selectAll("g")
      .data(nodes)
      .join("g")
      .style("cursor", (d: any) => d.type === 'layer' ? "pointer" : "default")
      .on("mouseenter", (event, d) => {
        if (d.type === 'layer') {
          const layerData = STRATEGIC_LAYERS.find(l => l.name === d.name);
          if (layerData && containerRef.current) {
            const containerRect = containerRef.current.getBoundingClientRect();
            setTooltip({
              visible: true,
              x: event.clientX - containerRect.left,
              y: event.clientY - containerRect.top,
              title: d.name,
              content: layerData.subLayers
            });
          }
        }
      })
      .on("mousemove", (event, d) => {
        if (d.type === 'layer' && containerRef.current) {
            const containerRect = containerRef.current.getBoundingClientRect();
            setTooltip(prev => ({
                ...prev,
                x: event.clientX - containerRect.left,
                y: event.clientY - containerRect.top
            }));
        }
      })
      .on("mouseleave", (event, d) => {
        if (d.type === 'layer') {
          setTooltip(prev => ({ ...prev, visible: false }));
        }
      })
      .call(d3.drag<SVGGElement, any>()
        .on("start", dragstarted)
        .on("drag", dragged)
        .on("end", dragended));

    nodeGroup.append("circle")
      .attr("r", (d: any) => d.radius)
      .attr("fill", (d: any) => d.type === 'root' ? "#10b981" : d.type === 'layer' ? "#06b6d4" : "#a259ff")
      .attr("stroke", "#090b14")
      .attr("stroke-width", 2)
      .style("filter", (d: any) => d.type === 'root' || d.type === 'layer' ? "url(#glow-arch)" : "none");

    const labelGroup = svg.append("g")
      .selectAll("text")
      .data(nodes)
      .join("text")
      .attr("text-anchor", "middle")
      .attr("dy", (d: any) => d.type === 'root' || d.type === 'layer' ? d.radius + 15 : d.radius + 10)
      .text((d: any) => d.name)
      .attr("fill", (d: any) => d.type === 'root' ? "#10b981" : d.type === 'layer' ? "#fff" : "#cbd5e1")
      .style("font-size", (d: any) => d.type === 'root' ? "14px" : d.type === 'layer' ? "12px" : "10px")
      .style("font-family", "mono")
      .style("font-weight", (d: any) => d.type === 'layer' ? "bold" : "normal")
      .style("pointer-events", "none")
      .style("text-shadow", "0 1px 3px rgba(0,0,0,0.8)")
      .clone(true).lower()
      .attr("stroke", "#090b14")
      .attr("stroke-width", 3);

    simulation.on("tick", () => {
      linkGroup
        .attr("x1", (d: any) => d.source.x)
        .attr("y1", (d: any) => d.source.y)
        .attr("x2", (d: any) => d.target.x)
        .attr("y2", (d: any) => d.target.y);

      nodeGroup
        .attr("transform", (d: any) => `translate(${Math.max(d.radius, Math.min(width - d.radius, d.x))},${Math.max(d.radius, Math.min(height - d.radius, d.y))})`);

      labelGroup
        .attr("x", (d: any) => d.x)
        .attr("y", (d: any) => d.y);
    });

    // Animation interval for live connection traffic simulation
    const trafficInterval = setInterval(() => {
        pulseLinks
          .attr("cx", (d: any) => d.source.x)
          .attr("cy", (d: any) => d.source.y)
          .style("opacity", () => Math.random() > 0.5 ? 0.8 : 0)
          .transition()
          .duration((d: any) => 1000 + Math.random() * 2000)
          .attr("cx", (d: any) => d.target.x)
          .attr("cy", (d: any) => d.target.y)
          .style("opacity", 0);
          
        nodeGroup.select("circle")
           .transition()
           .duration(1000)
           .attr("r", (d: any) => d.type === 'component' ? d.radius + (Math.random() * 2 - 1) * 2 : d.radius);

    }, 3000);

    function dragstarted(event: any) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      event.subject.fx = event.subject.x;
      event.subject.fy = event.subject.y;
    }

    function dragged(event: any) {
      event.subject.fx = event.x;
      event.subject.fy = event.y;
    }

    function dragended(event: any) {
      if (!event.active) simulation.alphaTarget(0);
      event.subject.fx = null;
      event.subject.fy = null;
    }
    
    return () => {
        clearInterval(trafficInterval);
        simulation.stop();
    };

  }, []);

  useEffect(() => {
    if (!svgRef.current) return;
    const svg = d3.select(svgRef.current);
    const q = searchQuery.toLowerCase().trim();

    svg.selectAll(".node-group g").each(function(d: any) {
      const isMatch = q && d.name.toLowerCase().includes(q);
      const group = d3.select(this);

      group.select("circle")
        .transition().duration(300)
        .attr("stroke", isMatch ? "#fde047" : "#090b14")
        .attr("stroke-width", isMatch ? 4 : 2)
        .style("opacity", (q && !isMatch && d.type !== 'root') ? 0.2 : 1)
        .style("filter", isMatch ? "drop-shadow(0 0 10px rgba(253, 224, 71, 0.8))" : (d.type === 'root' || d.type === 'layer' ? "url(#glow-arch)" : "none"));

      group.selectAll("text")
        .transition().duration(300)
        .style("opacity", (q && !isMatch && d.type !== 'root') ? 0.2 : 1)
        .attr("fill", isMatch ? "#fde047" : (d.type === 'root' ? "#10b981" : d.type === 'layer' ? "#fff" : "#cbd5e1"));
    });
  }, [searchQuery]);

  return (
    <div ref={containerRef} className="w-full h-full relative flex items-center justify-center">
      <svg ref={svgRef} className="w-full h-full" />
      {tooltip.visible && (
        <div 
          className="absolute z-50 pointer-events-none bg-[#090b14]/95 backdrop-blur-md border border-white/10 rounded-lg p-3 shadow-2xl transition-opacity duration-150 transform -translate-x-1/2 mt-4 max-w-[250px]"
          style={{ left: tooltip.x, top: tooltip.y }}
        >
          <h4 className="font-mono text-white text-xs font-bold mb-2 border-b border-white/10 pb-1">{tooltip.title}</h4>
          <ul className="flex flex-col gap-1">
            {tooltip.content.map((comp, idx) => (
              <li key={idx} className="text-[10px] text-slate-300 font-mono flex items-center gap-2">
                <span className="w-1.5 h-1.5 rounded-full bg-[#a259ff] inline-block animate-pulse" style={{ animationDelay: `${idx * 0.1}s` }} />
                {comp}
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
