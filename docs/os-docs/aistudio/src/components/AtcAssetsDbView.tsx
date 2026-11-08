import React, { useState, useEffect, useRef } from 'react';
import { Database, Image, Box, FileJson, LayoutTemplate, Layers, Search, Filter, Activity, Eye, X } from 'lucide-react';
import { PieChart, Pie, Cell, ResponsiveContainer } from 'recharts';
import * as THREE from 'three';

export function AtcAssetsDbView() {
  const assets = [
    { title: "Standard Token Interface", type: "JSON Spec", category: "Smart Contracts", subcategory: "Token Standards", tags: ["EIP", "Interface"], version: "v1.2.0" },
    { title: "A-Town Base Vector Logo", type: "SVG", category: "Brand Assets", subcategory: "Logos", tags: ["vector", "design"], version: "v2.0.1" },
    { title: "Node Architecture Diagram", type: "System", category: "Documentation", subcategory: "Architecture", tags: ["diagram", "nodes"], version: "v1.0.5" },
    { title: "Governance Voter Schema", type: "JSON Spec", category: "Smart Contracts", subcategory: "DAO", tags: ["schema", "voting"], version: "v3.1.0" },
    { title: "Wallet Mockup Library", type: "UI Asset", category: "Design System", subcategory: "Figma Components", tags: ["ui", "mockups"], version: "v4.0.0" },
    { title: "Network Configuration", type: "System", category: "Infrastructure", subcategory: "Config Files", tags: ["yaml", "settings"], version: "v1.5.2" },
    { title: "ATC Hover Node Render", type: "Mesh", category: "3D Assets", subcategory: "Nodes", tags: ["render", "3d"], version: "v1.0.0" }
  ];

  const [wellnessScore, setWellnessScore] = useState(85);
  const [previewMeshId, setPreviewMeshId] = useState<string | null>(null);
  
  useEffect(() => {
    const interval = setInterval(() => {
      setWellnessScore(prev => {
        const jump = (Math.random() - 0.5) * 5;
        return Math.min(100, Math.max(0, prev + jump));
      });
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  const gaugeData = [
    { name: 'Score', value: wellnessScore },
    { name: 'Remaining', value: 100 - wellnessScore }
  ];

  const getColor = (score: number) => {
    if (score >= 80) return '#10b981'; // emerald-500
    if (score >= 60) return '#f59e0b'; // amber-500
    return '#ef4444'; // red-500
  };

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans relative">
      <div className="mb-4 flex items-center justify-between gap-4">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-pink-500/10 border border-pink-500/30 flex items-center justify-center text-pink-400 shadow-[0_0_15px_rgba(236,72,153,0.2)]">
            <Database className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
              ATC-Lang Assets Datenbank
            </h2>
            <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
              Zentrale Repository für statische Assets, Spezifikationen und UI-Komponenten für ATC-Lang Entwickler.
            </p>
          </div>
        </div>
        
        {/* Repository Wellness Gauge */}
        <div className="w-64 p-4 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col items-center shadow-lg">
          <h3 className="text-xs font-semibold text-slate-400 font-mono tracking-widest uppercase mb-2 flex items-center gap-2">
            <Activity className="w-3.5 h-3.5" /> Repository Wellness
          </h3>
          <div className="h-24 w-full relative">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={gaugeData}
                  cx="50%"
                  cy="100%"
                  startAngle={180}
                  endAngle={0}
                  innerRadius={60}
                  outerRadius={80}
                  paddingAngle={0}
                  dataKey="value"
                  stroke="none"
                >
                  <Cell fill={getColor(wellnessScore)} />
                  <Cell fill="#1e293b" />
                </Pie>
              </PieChart>
            </ResponsiveContainer>
            <div className="absolute bottom-0 left-1/2 -translate-x-1/2 text-2xl font-bold font-mono" style={{ color: getColor(wellnessScore) }}>
              {Math.round(wellnessScore)}%
            </div>
          </div>
          <div className="flex justify-between w-full mt-2 px-6 text-[10px] text-slate-500">
            <span>PassRate</span>
            <span>Commits</span>
          </div>
        </div>
      </div>

      <div className="flex gap-4 mb-2">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
          <input type="text" placeholder="Search assets..." className="w-full bg-[#090b14] border border-white/10 rounded-lg py-2.5 pl-10 pr-4 text-sm text-white focus:outline-none focus:border-pink-500/50 transition-colors" />
        </div>
        <button className="px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-slate-300 text-sm hover:bg-white/10 transition-colors flex items-center gap-2">
          <Filter className="w-4 h-4" /> Types
        </button>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        {assets.map((asset, index) => (
          <div key={index} className="p-4 rounded-xl bg-black/40 border border-white/5 hover:border-pink-500/30 hover:bg-white/5 transition-all cursor-pointer flex flex-col items-center text-center gap-3 relative group">
             <div className="p-4 rounded-full bg-white/5 border border-white/10 group-hover:scale-110 transition-transform">
               {asset.type.includes("JSON") ? <FileJson className="w-6 h-6 text-amber-400" /> :
                asset.type.includes("UI") ? <LayoutTemplate className="w-6 h-6 text-blue-400" /> :
                asset.type.includes("SVG") ? <Image className="w-6 h-6 text-emerald-400" /> :
                asset.type.includes("Mesh") ? <Box className="w-6 h-6 text-pink-400" /> :
                <Layers className="w-6 h-6 text-slate-400" />
               }
             </div>
             <div>
               <h3 className="text-sm font-medium text-slate-200 mb-1 line-clamp-1">{asset.title}</h3>
               <span className="text-[9px] font-mono tracking-widest uppercase text-slate-500">{asset.type} • {asset.version}</span>
               <div className="flex flex-col gap-0.5 mt-2 text-[10px] text-slate-400">
                 <span className="text-pink-400/80">{asset.category}</span>
                 <span className="text-slate-500 text-[9px]">↳ {asset.subcategory}</span>
               </div>
               <div className="flex flex-wrap items-center justify-center gap-1 mt-2">
                 {asset.tags.map(t => (
                   <span key={t} className="px-1.5 py-0.5 rounded bg-white/5 border border-white/10 text-[8px] text-slate-300">{t}</span>
                 ))}
               </div>
             </div>
             {asset.type.includes("Mesh") && (
               <button 
                 onClick={(e) => {
                   e.stopPropagation();
                   setPreviewMeshId(asset.title);
                 }}
                 className="absolute inset-0 m-auto w-10 h-10 rounded-full bg-pink-500 text-white flex flex-col items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity translate-y-2 group-hover:translate-y-0 duration-300 shadow-xl"
               >
                 <Eye className="w-4 h-4" />
               </button>
             )}
          </div>
        ))}
      </div>

      {/* 3D Mesh Preview Modal */}
      {previewMeshId && (
        <ThreePreviewModal 
           assetName={previewMeshId} 
           onClose={() => setPreviewMeshId(null)} 
        />
      )}
    </div>
  );
}

function ThreePreviewModal({ assetName, onClose }: { assetName: string; onClose: () => void }) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!containerRef.current) return;
    const currentContainer = containerRef.current;
    
    // Scene Setup
    const scene = new THREE.Scene();
    scene.background = new THREE.Color('#050811');
    
    // Camera
    const camera = new THREE.PerspectiveCamera(45, currentContainer.clientWidth / currentContainer.clientHeight, 0.1, 1000);
    camera.position.z = 5;
    
    // Renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(currentContainer.clientWidth, currentContainer.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    currentContainer.appendChild(renderer.domElement);
    
    // Abstract Mesh
    const geometry = new THREE.IcosahedronGeometry(1.5, 1);
    const material = new THREE.MeshPhysicalMaterial({
      color: 0xec4899,
      wireframe: true,
      emissive: 0xec4899,
      emissiveIntensity: 0.2,
      roughness: 0.1,
      metalness: 0.8
    });
    const mesh = new THREE.Mesh(geometry, material);
    scene.add(mesh);
    
    // Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    
    const pointLight = new THREE.PointLight(0x00ffff, 1, 100);
    pointLight.position.set(5, 5, 5);
    scene.add(pointLight);

    const pointLight2 = new THREE.PointLight(0xff00ff, 1, 100);
    pointLight2.position.set(-5, -5, -5);
    scene.add(pointLight2);
    
    // Animation
    let animationFrameId: number;
    const animate = () => {
      animationFrameId = requestAnimationFrame(animate);
      mesh.rotation.x += 0.005;
      mesh.rotation.y += 0.01;
      renderer.render(scene, camera);
    };
    animate();
    
    // Handle resize
    const handleResize = () => {
       if (!currentContainer) return;
       camera.aspect = currentContainer.clientWidth / currentContainer.clientHeight;
       camera.updateProjectionMatrix();
       renderer.setSize(currentContainer.clientWidth, currentContainer.clientHeight);
    };
    window.addEventListener('resize', handleResize);
    
    return () => {
      cancelAnimationFrame(animationFrameId);
      window.removeEventListener('resize', handleResize);
      currentContainer.removeChild(renderer.domElement);
      geometry.dispose();
      material.dispose();
      renderer.dispose();
    };
  }, []);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="w-full max-w-4xl h-[70vh] bg-[#090b14] border border-white/10 rounded-2xl flex flex-col shadow-2xl relative overflow-hidden">
        <div className="flex items-center justify-between p-4 border-b border-white/10 bg-black/50 absolute top-0 left-0 right-0 z-10">
          <div className="flex items-center gap-3">
             <Box className="w-5 h-5 text-pink-400" />
             <h3 className="font-semibold text-white">{assetName} Preview</h3>
          </div>
          <button onClick={onClose} className="p-1 text-slate-400 hover:text-white hover:bg-white/10 rounded-lg transition-colors">
            <X className="w-5 h-5" />
          </button>
        </div>
        <div ref={containerRef} className="w-full h-full cursor-move" />
        <div className="absolute bottom-4 left-1/2 -translate-x-1/2 px-4 py-2 bg-black/50 border border-white/10 rounded-full text-xs font-mono text-slate-400 flex gap-4 backdrop-blur-md">
           <span>Vertices: {42}</span>
           <span>Faces: {80}</span>
           <span>Format: ATC-Mesh (.atm)</span>
        </div>
      </div>
    </div>
  );
}
