import React, { useState, useEffect, useRef } from 'react';
import { Network, Search, Hash, Clock, Box, Shield, Activity, Users, Server, X, RefreshCw } from 'lucide-react';
import { motion } from 'motion/react';
import * as d3 from 'd3';

function ForceDirectedNetworkMap() {
  const svgRef = useRef<SVGSVGElement>(null);
  const [selectedNode, setSelectedNode] = useState<any>(null);

  useEffect(() => {
    if (!svgRef.current) return;
    
    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const width = 800;
    const height = 300;

    // Use viewBox for responsiveness
    svg.attr("viewBox", `0 0 1000 500`);
    svg.style("width", "100%");
    svg.style("height", "100%");

    const nodeTypes = ['validator', 'miner', 'observer', 'bridge'];
    const nodes = Array.from({ length: 60 }, (_, i) => {
      const type = nodeTypes[Math.floor(Math.random() * nodeTypes.length)];
      return { 
        id: `node-${i}`, 
        group: i === 0 ? 1 : 2,
        type,
        syncStatus: Math.random() > 0.1 ? 'synced' : 'syncing',
        latency: Math.floor(Math.random() * 150) + 10,
        uptime: (Math.random() * 5 + 95).toFixed(2),
        connections: Math.floor(Math.random() * 8) + 2
      };
    });
    
    const links: any[] = [];
    nodes.forEach((node, i) => {
      const numLinks = Math.floor(Math.random() * 3) + 1;
      for (let j = 0; j < numLinks; j++) {
        const target = nodes[Math.floor(Math.random() * nodes.length)];
        if (target.id !== node.id) {
          links.push({ source: node.id, target: target.id });
        }
      }
    });

    const simulation = d3.forceSimulation(nodes as d3.SimulationNodeDatum[])
      .force("link", d3.forceLink(links).id((d: any) => d.id).distance(50))
      .force("charge", d3.forceManyBody().strength(-120))
      .force("center", d3.forceCenter(1000 / 2, 500 / 2))
      .force("collide", d3.forceCollide().radius(25));

    // Glow filter
    const defs = svg.append("defs");
    const filter = defs.append("filter").attr("id", "explorer-glow");
    filter.append("feGaussianBlur").attr("stdDeviation", "2.5").attr("result", "coloredBlur");
    const feMerge = filter.append("feMerge");
    feMerge.append("feMergeNode").attr("in", "coloredBlur");
    feMerge.append("feMergeNode").attr("in", "SourceGraphic");

    const link = svg.append("g")
      .attr("stroke", "#334155")
      .attr("stroke-opacity", 0.4)
      .selectAll("line")
      .data(links)
      .join("line")
      .attr("stroke-width", 1.5);

    const nodeGroup = svg.append("g")
      .selectAll("g")
      .data(nodes)
      .join("g")
      .style("cursor", "pointer")
      .call(d3.drag<SVGGElement, any>()
        .on("start", dragstarted)
        .on("drag", dragged)
        .on("end", dragended))
      .on("click", (event, d) => {
        setSelectedNode(d);
        event.stopPropagation();
      });

    svg.on("click", () => {
      setSelectedNode(null);
    });

    // Outer ring for sync status
    nodeGroup.append("circle")
      .attr("r", 15)
      .attr("fill", "transparent")
      .attr("stroke", (d: any) => d.syncStatus === 'synced' ? '#10b981' : '#f59e0b')
      .attr("stroke-width", 2)
      .attr("stroke-dasharray", (d: any) => d.syncStatus === 'synced' ? 'none' : '2 2')
      .style("filter", "url(#explorer-glow)");

    // Inner circle
    nodeGroup.append("circle")
      .attr("r", 9)
      .attr("fill", (d: any) => {
        switch (d.type) {
          case 'validator': return '#6366f1';
          case 'miner': return '#ec4899';
          case 'bridge': return '#06b6d4';
          default: return '#64748b';
        }
      });

    nodeGroup.append("title")
      .text((d: any) => d.id);

    simulation.on("tick", () => {
      link
        .attr("x1", (d: any) => d.source.x)
        .attr("y1", (d: any) => d.source.y)
        .attr("x2", (d: any) => d.target.x)
        .attr("y2", (d: any) => d.target.y);

      nodeGroup.attr("transform", (d: any) => `translate(${d.x},${d.y})`);
    });

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
      simulation.stop();
    };
  }, []);

  return (
    <div className="w-full h-full relative">
       {selectedNode && (
         <div className="absolute top-2 right-2 z-20 w-64 bg-[#0a0d17]/95 backdrop-blur-xl border border-indigo-500/30 rounded-xl p-4 shadow-[0_0_30px_rgba(99,102,241,0.15)] flex flex-col pointer-events-auto text-left">
           <div className="flex justify-between items-center mb-3">
             <div className="flex items-center gap-2">
               <Server className="w-3 h-3 text-indigo-400" />
               <span className="text-xs font-bold uppercase tracking-wide text-slate-200">Node Details</span>
             </div>
             <button onClick={() => setSelectedNode(null)} className="text-slate-500 hover:text-white transition-colors">
               <X className="w-3 h-3" />
             </button>
           </div>
           <div className="flex flex-col gap-2">
             <div className="flex justify-between items-center border-b border-white/5 pb-1">
               <span className="text-[10px] font-mono text-slate-400">ID</span>
               <span className="text-[10px] font-mono text-white">{selectedNode.id}</span>
             </div>
             <div className="flex justify-between items-center border-b border-white/5 pb-1">
               <span className="text-[10px] font-mono text-slate-400">Typ</span>
               <span className="text-[10px] font-mono text-indigo-300 capitalize">{selectedNode.type}</span>
             </div>
             <div className="flex justify-between items-center border-b border-white/5 pb-1">
               <span className="text-[10px] font-mono flex items-center gap-1 text-slate-400"><Activity className="w-2.5 h-2.5" /> Latenz</span>
               <span className="text-[10px] font-mono text-emerald-400">{selectedNode.latency} ms</span>
             </div>
             <div className="flex justify-between items-center border-b border-white/5 pb-1">
               <span className="text-[10px] font-mono flex items-center gap-1 text-slate-400"><Clock className="w-2.5 h-2.5" /> Betriebszeit</span>
               <span className="text-[10px] font-mono text-cyan-400">{selectedNode.uptime}%</span>
             </div>
             <div className="flex justify-between items-center border-b border-white/5 pb-1">
               <span className="text-[10px] font-mono flex items-center gap-1 text-slate-400"><Users className="w-2.5 h-2.5" /> Peer-Verbindungen</span>
               <span className="text-[10px] font-mono text-white">{selectedNode.connections}</span>
             </div>
             <div className="flex justify-between items-center">
               <span className="text-[10px] font-mono flex items-center gap-1 text-slate-400"><RefreshCw className="w-2.5 h-2.5" /> Status</span>
               <span className={`text-[10px] font-mono font-bold ${selectedNode.syncStatus === 'synced' ? 'text-emerald-500' : 'text-amber-500'}`}>{selectedNode.syncStatus}</span>
             </div>
           </div>
         </div>
       )}
       <svg ref={svgRef} className="w-full h-full min-h-[300px] cursor-grab active:cursor-grabbing" />
    </div>
  );
}

export function NetworkExplorerView() {
  const [search, setSearch] = useState('');
  const [networkInfo, setNetworkInfo] = useState<any>(null);
  const [blocks, setBlocks] = useState<any[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const infoRes = await fetch('/api/blockchain/info');
        if (infoRes.ok) {
          setNetworkInfo(await infoRes.json());
        }
        const blocksRes = await fetch('/api/blockchain/blocks');
        if (blocksRes.ok) {
          const allBlocks = await blocksRes.json();
          // reverse to show newest first
          setBlocks(allBlocks.reverse().slice(0, 10));
        }
      } catch (e) {
        console.error('Failed to fetch blockchain data', e);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 4000);
    return () => clearInterval(interval);
  }, []);

  const calculateTps = () => {
    if (!networkInfo) return '0 TPS';
    return '18-45 TPS';
  };

  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 w-full max-w-6xl mx-auto">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-4">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
            <Network className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">ATC Blockchain Daten & Statistiken</h2>
            <p className="text-sm font-light text-slate-400">Echtzeit-Ansicht der Blöcke, Transaktionen und Validator-Nodes im ATC Mainnet</p>
          </div>
        </div>

        <div className="relative w-full md:w-[350px]">
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Search className="h-4 w-4 text-slate-500" />
          </div>
          <input
            type="text"
            className="block w-full pl-10 pr-3 py-2 border border-atc-border/50 rounded-xl leading-5 bg-[#090b14]/80 text-slate-300 placeholder-slate-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm transition-colors"
            placeholder="Suche nach Blöcken, Transaktionen, Adressen..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
      </div>

      {/* Network Stats */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-4">
        {[
           { label: 'Letzter Block', value: networkInfo?.height || '...', icon: Box, color: 'text-indigo-400', bg: 'bg-indigo-500/10' },
           { label: 'Durchschn. Blockzeit', value: '10.0s', icon: Clock, color: 'text-atc-cyan', bg: 'bg-atc-cyan/10' },
           { label: 'Aktive Validatoren', value: networkInfo?.peers?.length || '...', icon: Shield, color: 'text-atc-purple', bg: 'bg-atc-purple/10' },
           { label: 'Netzwerk TPS', value: calculateTps(), icon: Activity, color: 'text-emerald-400', bg: 'bg-emerald-500/10' },
        ].map((stat, i) => (
          <div key={i} className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl p-5 flex items-center gap-4 hover:border-slate-600 transition-colors">
            <div className={`p-3 rounded-xl ${stat.bg}`}>
               <stat.icon className={`w-5 h-5 ${stat.color}`} />
            </div>
            <div>
               <div className="text-[10px] text-slate-500 font-mono font-bold tracking-widest uppercase mb-1">{stat.label}</div>
               <div className="text-xl font-semibold text-white">{stat.value}</div>
            </div>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-[#090b14]/60 border border-atc-border/50 rounded-2xl overflow-hidden">
          <div className="px-6 py-4 border-b border-atc-border/50 bg-[#060a16] flex justify-between items-center">
             <h3 className="text-lg font-medium text-white flex items-center gap-2">
               <Box className="w-5 h-5 text-slate-500" /> Letzte Blöcke
             </h3>
             <button className="text-sm text-indigo-400 hover:text-indigo-300 font-medium transition-colors">Alle ansehen</button>
          </div>
          <div className="divide-y divide-atc-border/30">
             {blocks.map((block, i) => (
               <div key={i} className="px-6 py-4 hover:bg-[#060a16]/50 transition-colors flex items-center justify-between group">
                  <div className="flex items-center gap-4">
                     <div className="w-10 h-10 rounded-lg bg-atc-border/20 flex items-center justify-center text-slate-400">
                        <Hash className="w-5 h-5" />
                     </div>
                     <div>
                        <div className="text-indigo-400 font-mono font-bold hover:underline cursor-pointer">Block #{block.index}</div>
                        <div className="text-xs text-slate-500 mt-1">{new Date(block.timestamp).toLocaleTimeString()}</div>
                     </div>
                  </div>
                  <div className="hidden sm:block text-left min-w-[120px]">
                     <div className="text-slate-300 font-mono text-sm">Miner / Validator</div>
                     <div className="text-atc-cyan text-xs font-mono truncate">{block.validator}</div>
                  </div>
              <div className="text-right flex flex-col items-end">
                     <div className="text-white text-sm font-medium">{block.transactions.length} Txns</div>
                     <div className="text-slate-500 text-[10px] mt-1 font-mono">
                        {block.consensusMetrics ? (
                          <div className="flex gap-2">
                             <span title={`PoI: ${block.consensusMetrics.poI_score}`} className="text-pink-400/80">PoI:{block.consensusMetrics.poI_score}</span>
                             <span title={`PoS: ${block.consensusMetrics.poS_stakeWeight}`} className="text-indigo-400/80">PoS:{block.consensusMetrics.poS_stakeWeight}</span>
                             <span title={`PoW Diff: ${block.consensusMetrics.poW_difficulty}`} className="text-atc-cyan/80">PoW:{block.consensusMetrics.poW_difficulty}</span>
                          </div>
                        ) : (
                          <span className="w-[100px] truncate block">{block.hash}</span>
                        )}
                     </div>
                  </div>
               </div>
             ))}
             {blocks.length === 0 && (
               <div className="px-6 py-8 text-center text-slate-500">Verbinde zur Chain...</div>
             )}
          </div>
        </div>

          <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl overflow-hidden flex flex-col">
            <div className="px-6 py-4 border-b border-atc-border/50 bg-[#060a16]">
               <h3 className="text-lg font-medium text-white flex items-center gap-2">
                 <Users className="w-5 h-5 text-slate-500" /> Netzwerk-Verteilung
               </h3>
            </div>
            <div className="p-6">
               <div className="space-y-4">
                 <div className="flex justify-between items-center bg-black/40 p-3 rounded-lg border border-white/5">
                   <div className="text-sm text-slate-400">Validatoren</div>
                   <div className="text-sm font-mono text-indigo-400 font-bold">18</div>
                 </div>
                 <div className="flex justify-between items-center bg-black/40 p-3 rounded-lg border border-white/5">
                   <div className="text-sm text-slate-400">Miner</div>
                   <div className="text-sm font-mono text-pink-400 font-bold">12</div>
                 </div>
                 <div className="flex justify-between items-center bg-black/40 p-3 rounded-lg border border-white/5">
                   <div className="text-sm text-slate-400">Bridges</div>
                   <div className="text-sm font-mono text-cyan-400 font-bold">10</div>
                 </div>
                 <div className="pt-4 border-t border-white/10 text-center">
                    <div className="text-emerald-400 font-mono text-sm tracking-widest uppercase font-bold mb-2">Netzwerk {networkInfo ? 'Gesund' : 'Verbindet'}</div>
                    <div className="text-slate-500 text-xs">Nodes nehmen aktiv am PoH + PoW + PoS + PoI Konsens teil.</div>
                 </div>
               </div>
            </div>
          </div>
        </div>

        <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl overflow-hidden shadow-2xl mt-6 relative">
          <div className="px-6 py-4 border-b border-atc-border/50 bg-[#060a16]">
             <h3 className="text-lg font-medium text-white flex items-center gap-2">
               <Network className="w-5 h-5 text-slate-500" /> Interaktive Netzwerk-Topologie
             </h3>
          </div>
          <div className="p-6 relative min-h-[500px] flex items-center justify-center">
             <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-indigo-500/5 via-transparent to-transparent pointer-events-none" />
             <div className="absolute left-6 top-6 flex flex-col gap-2 z-10 bg-black/50 backdrop-blur border border-white/10 rounded-lg p-3 pointer-events-none">
               <div className="flex items-center gap-2 font-mono text-xs font-bold uppercase tracking-wide text-slate-300">
                 Legende
               </div>
               <div className="flex items-center gap-2 text-xs text-slate-400 font-mono"><div className="w-3 h-3 rounded-full bg-indigo-500" /> Validatoren</div>
               <div className="flex items-center gap-2 text-xs text-slate-400 font-mono"><div className="w-3 h-3 rounded-full bg-pink-500" /> Miner</div>
               <div className="flex items-center gap-2 text-xs text-slate-400 font-mono"><div className="w-3 h-3 rounded-full bg-cyan-500" /> Bridges</div>
             </div>
             
             <div className="w-full h-[500px]">
                <ForceDirectedNetworkMap />
             </div>
          </div>
        </div>
      </div>
    );
  }
