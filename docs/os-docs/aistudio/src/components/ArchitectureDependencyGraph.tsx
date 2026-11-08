import React, { useEffect, useRef } from "react";
import * as d3 from "d3";
import { LAYERS } from "../data";

export function ArchitectureDependencyGraph() {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current) return;

    const width = 1000;
    const height = 800;

    // Clear previous
    d3.select(svgRef.current).selectAll("*").remove();

    const svg = d3
      .select(svgRef.current)
      .attr("viewBox", [-width / 2, -height / 2, width, height])
      .attr("width", "100%")
      .attr("height", "100%")
      .style("background", "transparent");

    const nodes: any[] = [];
    const links: any[] = [];

    // Root node
    const rootId = "A-TownChain Core";
    nodes.push({ id: rootId, label: rootId, group: 0, radius: 24 });

    LAYERS.forEach((layer) => {
      const layerId = `L${layer.id}: ${layer.name}`;
      nodes.push({
        id: layerId,
        label: layer.name,
        group: 1,
        radius: 14,
        layerId: layer.id,
      });

      // Link layer to the core
      links.push({ source: rootId, target: layerId, distance: 200, value: 1 });

      // Link previous layer to this layer to form a sequential backbone
      if (layer.id > 1) {
        const prevLayer = LAYERS.find((l) => l.id === layer.id - 1);
        if (prevLayer) {
          const prevLayerId = `L${prevLayer.id}: ${prevLayer.name}`;
          links.push({
            source: prevLayerId,
            target: layerId,
            distance: 150,
            value: 0.5,
          });
        }
      }

      layer.subLayers.forEach((comp) => {
        const compId = `${layer.id}-${comp}`;
        // Add component node
        nodes.push({ id: compId, label: comp, group: 2, radius: 6 });
        // Link component to its layer
        links.push({ source: layerId, target: compId, distance: 60, value: 1 });
      });
    });

    const simulation = d3
      .forceSimulation(nodes)
      .force(
        "link",
        d3
          .forceLink(links)
          .id((d: any) => d.id)
          .distance((d: any) => d.distance),
      )
      .force("charge", d3.forceManyBody().strength(-80))
      .force("center", d3.forceCenter(0, 0))
      .force(
        "collide",
        d3
          .forceCollide()
          .radius((d: any) => d.radius + 15)
          .iterations(2),
      );

    const link = svg
      .append("g")
      .attr("stroke", "rgba(255, 255, 255, 0.1)")
      .selectAll("line")
      .data(links)
      .join("line")
      .attr("stroke-width", (d: any) => d.value * 1.5)
      .attr("stroke-dasharray", (d: any) => (d.value === 0.5 ? "4,4" : "none")); // Dashed lines for sequential dependencies

    const nodeGroup = svg
      .append("g")
      .selectAll("g")
      .data(nodes)
      .join("g")
      .call(
        d3
          .drag()
          .on("start", dragstarted)
          .on("drag", dragged)
          .on("end", dragended) as any,
      );

    const tooltip = d3
      .select(svgRef.current.parentNode as HTMLElement)
      .append("div")
      .style("opacity", 0)
      .attr(
        "class",
        "absolute bg-[#090b14]/95 border border-atc-cyan/30 text-white px-3 py-2 rounded-lg pointer-events-none text-xs transform -translate-x-1/2 -translate-y-full mb-2 backdrop-blur-md z-10 whitespace-nowrap transition-opacity shadow-xl font-mono",
      );

    nodeGroup
      .append("circle")
      .attr("r", (d) => d.radius)
      .attr("fill", (d) =>
        d.group === 0 ? "#10b981" : d.group === 1 ? "#06b6d4" : "#6366f1",
      )
      .attr("stroke", (d) =>
        d.group === 0
          ? "rgba(16, 185, 129, 0.4)"
          : d.group === 1
            ? "rgba(6, 182, 212, 0.4)"
            : "rgba(99, 102, 241, 0.4)",
      )
      .attr("stroke-width", (d) => (d.group === 0 ? 6 : d.group === 1 ? 4 : 2))
      .on("mouseover", function (event, d: any) {
        d3.select(this)
          .attr("stroke-width", d.group === 0 ? 8 : d.group === 1 ? 6 : 4)
          .attr(
            "stroke",
            d.group === 0
              ? "rgba(16, 185, 129, 0.8)"
              : d.group === 1
                ? "rgba(6, 182, 212, 0.8)"
                : "rgba(99, 102, 241, 0.8)",
          );

        let labelText = `<div class="font-bold text-sm mb-1">${d.label}</div>`;
        if (d.group === 1) {
          labelText += `<div class="text-atc-cyan">Layer ${d.layerId}</div>`;
        } else if (d.group === 2) {
          labelText += `<div class="text-indigo-400">Component</div>`;
        } else {
          labelText += `<div class="text-emerald-400">System Core</div>`;
        }

        tooltip.transition().duration(200).style("opacity", 1);
        tooltip
          .html(labelText)
          .style("left", event.pageX + "px")
          .style("top", event.pageY - 10 + "px");
      })
      .on("mouseout", function (event, d: any) {
        d3.select(this)
          .attr("stroke-width", d.group === 0 ? 6 : d.group === 1 ? 4 : 2)
          .attr(
            "stroke",
            d.group === 0
              ? "rgba(16, 185, 129, 0.4)"
              : d.group === 1
                ? "rgba(6, 182, 212, 0.4)"
                : "rgba(99, 102, 241, 0.4)",
          );

        tooltip.transition().duration(500).style("opacity", 0);
      });

    nodeGroup
      .append("text")
      .attr("dy", "0.31em")
      .attr("x", (d) => d.radius + 8)
      .text((d) => (d.group !== 2 ? d.label : ""))
      .style("fill", (d) => (d.group === 0 ? "#10b981" : "#cbd5e1"))
      .style("font-size", (d) => (d.group === 0 ? "16px" : "12px"))
      .style("font-family", "monospace")
      .style("font-weight", (d) => (d.group === 0 ? "bold" : "normal"))
      .style("pointer-events", "none")
      .style("text-shadow", "0 2px 4px rgba(0,0,0,1)");

    simulation.on("tick", () => {
      // Keep nodes within the bounding box
      nodes.forEach((d) => {
        d.x = Math.max(-width / 2 + 20, Math.min(width / 2 - 200, d.x));
        d.y = Math.max(-height / 2 + 20, Math.min(height / 2 - 20, d.y));
      });

      link
        .attr("x1", (d: any) => d.source.x)
        .attr("y1", (d: any) => d.source.y)
        .attr("x2", (d: any) => d.target.x)
        .attr("y2", (d: any) => d.target.y);

      nodeGroup.attr("transform", (d: any) => `translate(${d.x},${d.y})`);
    });

    // Semantic zoom
    const zoom = d3
      .zoom()
      .scaleExtent([0.1, 4])
      .on("zoom", (event) => {
        nodeGroup.attr("transform", (d: any) => {
          const tr = event.transform;
          return `translate(${tr.applyX(d.x)}, ${tr.applyY(d.y)}) scale(${tr.k})`;
        });
        link
          .attr("x1", (d: any) => event.transform.applyX(d.source.x))
          .attr("y1", (d: any) => event.transform.applyY(d.source.y))
          .attr("x2", (d: any) => event.transform.applyX(d.target.x))
          .attr("y2", (d: any) => event.transform.applyY(d.target.y));
      });

    // svg.call(zoom as any);

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
      // Remove tooltip
      tooltip.remove();
    };
  }, []);

  return (
    <div className="w-full h-full relative cursor-move">
      <svg ref={svgRef} className="w-full h-full" />
    </div>
  );
}
