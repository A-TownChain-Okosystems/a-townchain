import React, { useState, useEffect, useRef } from 'react';
import { Search, Database, Plus, Trash2, GitBranch, Star, Layers, Activity, Maximize2 } from 'lucide-react';
import * as d3 from 'd3';

interface RepoNode {
  id: string;
  name: string;
  owner?: string;
  description?: string;
  stars?: number;
  type: 'repo' | 'ecosystem' | 'module';
  x?: number;
  y?: number;
  vx?: number;
  vy?: number;
}

interface RepoLink {
  source: string | RepoNode;
  target: string | RepoNode;
  value: number;
}

export function EcosystemVisualizerView() {
  const [repoQuery, setRepoQuery] = useState('');
  const [nodes, setNodes] = useState<RepoNode[]>([]);
  const [links, setLinks] = useState<RepoLink[]>([]);
  const [isMerged, setIsMerged] = useState(false);
  const [loading, setLoading] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const fetchRepo = async () => {
    if (!repoQuery || !repoQuery.includes('/')) return;
    
    setLoading(true);
    try {
      const res = await fetch(`https://api.github.com/repos/${repoQuery}`);
      if (res.ok) {
        const data = await res.json();
        if (!nodes.find(n => n.id === data.node_id)) {
          setNodes(prev => [...prev, {
            id: data.node_id,
            name: data.full_name,
            owner: data.owner.login,
            description: data.description,
            stars: data.stargazers_count,
            type: 'repo'
          }]);
        }
      }
    } catch(e) {
      console.error(e);
    } finally {
      setLoading(false);
      setRepoQuery('');
    }
  };

  const removeRepo = (id: string) => {
    setNodes(prev => prev.filter(n => n.id !== id));
    setLinks(prev => prev.filter(l => {
      const sourceId = typeof l.source === 'object' ? l.source.id : l.source;
      const targetId = typeof l.target === 'object' ? l.target.id : l.target;
      return sourceId !== id && targetId !== id;
    }));
  };

  const mergeIntoEcosystem = () => {
    if (nodes.length === 0) return;
    
    // Create central ecosystem node
    const ecoNode: RepoNode = {
      id: 'atown-ecosystem',
      name: 'A-TownChain Ecosystem',
      type: 'ecosystem'
    };
    
    const newLinks = nodes.map(n => ({
      source: n.id,
      target: ecoNode.id,
      value: 1
    }));
    
    setNodes(prev => [...prev, ecoNode]);
    setLinks(newLinks);
    setIsMerged(true);
  };

  const resetEcosystem = () => {
    setNodes(prev => prev.filter(n => n.type !== 'ecosystem'));
    setLinks([]);
    setIsMerged(false);
  };

  useEffect(() => {
    if (!containerRef.current) return;

    const width = containerRef.current.clientWidth;
    const height = containerRef.current.clientHeight;

    d3.select(containerRef.current).selectAll('*').remove();

    const svg = d3.select(containerRef.current)
      .append('svg')
      .attr('width', width)
      .attr('height', height)
      .attr('viewBox', [0, 0, width, height]);

    // Graph data deep copy to prevent d3 from mutating state directly
    const graphNodes = nodes.map(d => ({...d}));
    const graphLinks = links.map(d => ({
      source: typeof d.source === 'object' ? d.source.id : d.source,
      target: typeof d.target === 'object' ? d.target.id : d.target,
      value: d.value
    }));

    const simulation = d3.forceSimulation(graphNodes as any)
      .force('link', d3.forceLink(graphLinks).id((d: any) => d.id).distance(150))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide().radius(50));

    // Glow filter
    const defs = svg.append('defs');
    const filter = defs.append('filter').attr('id', 'glow');
    filter.append('feGaussianBlur').attr('stdDeviation', '4').attr('result', 'coloredBlur');
    const feMerge = filter.append('feMerge');
    feMerge.append('feMergeNode').attr('in', 'coloredBlur');
    feMerge.append('feMergeNode').attr('in', 'SourceGraphic');

    const link = svg.append('g')
      .selectAll('line')
      .data(graphLinks)
      .join('line')
      .attr('stroke', '#0ea5e9')
      .attr('stroke-opacity', 0.4)
      .attr('stroke-width', 2);

    const node = svg.append('g')
      .selectAll('g')
      .data(graphNodes)
      .join('g')
      .call(d3.drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended) as any);

    node.append('circle')
      .attr('r', (d: any) => d.type === 'ecosystem' ? 40 : 25)
      .attr('fill', (d: any) => d.type === 'ecosystem' ? '#10b981' : '#3b82f6')
      .attr('fill-opacity', 0.8)
      .attr('stroke', (d: any) => d.type === 'ecosystem' ? '#059669' : '#2563eb')
      .attr('stroke-width', 3)
      .style('filter', 'url(#glow)');

    node.append('text')
      .attr('dx', 0)
      .attr('dy', (d: any) => d.type === 'ecosystem' ? 60 : 40)
      .attr('text-anchor', 'middle')
      .text((d: any) => d.name)
      .attr('fill', '#e2e8f0')
      .attr('font-family', 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace')
      .attr('font-size', (d: any) => d.type === 'ecosystem' ? '14px' : '12px')
      .attr('font-weight', (d: any) => d.type === 'ecosystem' ? 'bold' : 'normal');

    simulation.on('tick', () => {
      link
        .attr('x1', (d: any) => d.source.x)
        .attr('y1', (d: any) => d.source.y)
        .attr('x2', (d: any) => d.target.x)
        .attr('y2', (d: any) => d.target.y);

      node
        .attr('transform', (d: any) => `translate(${d.x},${d.y})`);
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
  }, [nodes, links]);

  return (
    <div className="flex flex-col h-full bg-[#04060b] text-white">
      {/* Header */}
      <div className="p-6 border-b border-white/10 flex items-center justify-between shrink-0 bg-[#0b0f19]">
        <div>
          <h1 className="text-2xl font-mono tracking-tight flex items-center gap-3">
            <Layers className="w-6 h-6 text-emerald-400" />
            Ecosystem Builder
          </h1>
          <p className="text-sm font-mono text-slate-400 mt-1 flex items-center gap-2">
            Merge multiple repositories into a unified A-TownChain ecosystem visualization.
          </p>
        </div>
        {isMerged ? (
          <button 
            onClick={resetEcosystem}
            className="px-5 py-2.5 bg-slate-800 hover:bg-slate-700 border border-slate-600 rounded-xl font-mono text-sm tracking-wider uppercase transition-colors flex items-center gap-2"
          >
            Reset Layout
          </button>
        ) : (
          <button 
            onClick={mergeIntoEcosystem}
            disabled={nodes.length < 2}
            className="px-5 py-2.5 bg-emerald-500/20 text-emerald-400 hover:bg-emerald-500/30 border border-emerald-500/50 rounded-xl font-mono text-sm tracking-wider uppercase transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Database className="w-5 h-5" />
            Assemble Ecosystem
          </button>
        )}
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar */}
        <div className="w-[350px] border-r border-white/10 bg-[#0b0f19] p-6 flex flex-col gap-6 overflow-y-auto custom-scrollbar shrink-0">
          <div className="flex flex-col gap-3">
            <h3 className="font-mono text-sm text-slate-400 uppercase tracking-wider flex items-center gap-2">
              <Plus className="w-4 h-4" />
              Add Repository
            </h3>
            <div className="flex gap-2">
              <div className="relative flex-1">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
                <input 
                  type="text" 
                  value={repoQuery}
                  onChange={e => setRepoQuery(e.target.value)}
                  onKeyDown={e => e.key === 'Enter' && fetchRepo()}
                  placeholder="owner/repo..."
                  className="w-full bg-black/40 border border-white/10 rounded-xl py-2.5 pl-9 pr-4 text-sm font-mono focus:outline-none focus:border-emerald-500/50"
                  disabled={isMerged}
                />
              </div>
              <button 
                onClick={fetchRepo}
                disabled={!repoQuery || isMerged}
                className="p-2.5 bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 rounded-xl hover:bg-indigo-500/30 disabled:opacity-50"
              >
                <Plus className="w-5 h-5" />
              </button>
            </div>
          </div>

          <div className="flex-1 flex flex-col gap-3">
            <h3 className="font-mono text-sm text-slate-400 uppercase tracking-wider flex items-center justify-between">
              <span>Repositories</span>
              <span className="bg-white/10 px-2 py-0.5 rounded-full text-xs text-slate-300">
                {nodes.filter(n => n.type === 'repo').length}
              </span>
            </h3>
            
            <div className="flex flex-col gap-2">
              {nodes.filter(n => n.type === 'repo').map(node => (
                <div key={node.id} className="flex items-center justify-between bg-black/30 border border-white/5 p-3 rounded-xl group hover:border-white/10 transition-colors">
                  <div className="flex items-center gap-3 overflow-hidden">
                    <div className="w-8 h-8 rounded-lg bg-indigo-500/20 border border-indigo-500/30 flex items-center justify-center shrink-0">
                      <GitBranch className="w-4 h-4 text-indigo-400" />
                    </div>
                    <div className="min-w-0 pr-2">
                      <h4 className="font-mono text-sm text-white truncate">{node.name}</h4>
                      <p className="font-mono text-[10px] text-slate-500 truncate">github.com/{node.id}</p>
                    </div>
                  </div>
                  {!isMerged && (
                    <button 
                      onClick={() => removeRepo(node.id)}
                      className="p-1.5 text-slate-500 hover:text-rose-400 hover:bg-rose-500/10 rounded-lg transition-colors opacity-0 group-hover:opacity-100 shrink-0"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  )}
                </div>
              ))}
              
              {nodes.filter(n => n.type === 'repo').length === 0 && (
                <div className="py-8 text-center text-slate-500 font-mono flex flex-col items-center gap-2">
                  <Database className="w-8 h-8 opacity-50" />
                  No repositories added
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Main Canvas */}
        <div className="flex-1 relative bg-gradient-to-br from-slate-950 via-[#0a0f1c] to-indigo-950/20">
          <div ref={containerRef} className="absolute inset-0" />
          
          {/* Legend Overlay */}
          <div className="absolute bottom-6 left-6 bg-black/60 backdrop-blur-md border border-white/10 p-4 rounded-xl flex flex-col gap-3 pointer-events-none">
            <h4 className="font-mono text-xs text-slate-400 uppercase tracking-widest font-bold">Legend</h4>
            <div className="flex items-center gap-3">
              <div className="w-3 h-3 rounded-full bg-blue-500 border border-blue-400 shrink-0 shadow-[0_0_8px_rgba(59,130,246,0.8)]" />
              <span className="font-mono text-xs text-white">Repository</span>
            </div>
            {isMerged && (
              <div className="flex items-center gap-3">
                <div className="w-3 h-3 rounded-full bg-emerald-500 border border-emerald-400 shrink-0 shadow-[0_0_8px_rgba(16,185,129,0.8)]" />
                <span className="font-mono text-xs text-white">Ecosystem Hub</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
