import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';

interface Props {
  language?: 'EN' | 'DE';
}

const translations = {
  EN: {
    success: 'Success',
    failed: 'Failed'
  },
  DE: {
    success: 'Erfolgreich',
    failed: 'Fehlgeschlagen'
  }
};

export function SyncStatusDonutChart({ language = 'DE' }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const t = translations[language];

  useEffect(() => {
    if (!containerRef.current) return;

    // Data definition
    const data = [
      { status: 'success', value: 85, color: '#10b981' }, // emerald-500
      { status: 'error', value: 15, color: '#f43f5e' }    // rose-500
    ];

    const width = 140;
    const height = 140;
    const margin = 10;
    const radius = Math.min(width, height) / 2 - margin;

    // Clear prev
    d3.select(containerRef.current).selectAll('*').remove();

    const svg = d3.select(containerRef.current)
      .append('svg')
      .attr('width', width)
      .attr('height', height)
      .append('g')
      .attr('transform', `translate(${width / 2},${height / 2})`);

    const color = d3.scaleOrdinal()
      .domain(data.map(d => d.status))
      .range(data.map(d => d.color));

    const pie = d3.pie<any>()
      .value(d => d.value)
      .sort(null);

    const arc = d3.arc<any>()
      .innerRadius(radius * 0.65)
      .outerRadius(radius);

    const tooltip = d3.select(containerRef.current)
      .append('div')
      .style('opacity', 0)
      .attr('class', 'absolute bg-[#00101c]/95 border border-atc-cyan/30 text-white px-2 py-1 rounded pointer-events-none text-xs transform -translate-x-1/2 -translate-y-full mb-1 backdrop-blur-md z-10 whitespace-nowrap transition-opacity');

    svg.selectAll('path')
      .data(pie(data))
      .enter()
      .append('path')
      .attr('d', arc)
      .attr('fill', d => color(d.data.status) as string)
      .attr('stroke', '#090b14') // Background color
      .style('stroke-width', '2px')
      .style('opacity', 0.9)
      .on('mouseover', function(event, d) {
        d3.select(this).style('opacity', 1);
        tooltip.transition().duration(200).style('opacity', 1);
        const label = d.data.status === 'success' ? t.success : t.failed;
        tooltip.html(`<div class="font-bold">${label}:</div><div>${d.data.value}%</div>`)
          .style('left', (event.pageX) + 'px')
          .style('top', (event.pageY) + 'px');
      })
      .on('mouseout', function() {
        d3.select(this).style('opacity', 0.9);
        tooltip.transition().duration(500).style('opacity', 0);
      });

    // Center text
    svg.append('text')
      .attr('text-anchor', 'middle')
      .attr('dy', '0.3em')
      .attr('font-size', '18px')
      .attr('font-weight', 'bold')
      .attr('fill', '#ffffff')
      .attr('class', 'font-mono')
      .text('30D');

  }, [language, t]);

  return <div className="relative" ref={containerRef} />;
}
