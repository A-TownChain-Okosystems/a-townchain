import React, { useState, useEffect, useRef } from 'react';
import { Pencil, Square, Circle, Box, Rotate3D, Download, Trash, Maximize } from 'lucide-react';
import * as THREE from 'three';

export function Paint3DView() {
  const [activeTab, setActiveTab] = useState<'2d' | '3d'>('3d');
  const canvasRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (activeTab !== '3d' || !canvasRef.current) return;

    // Vanilla Three.js Setup
    const scene = new THREE.Scene();
    scene.background = new THREE.Color('#1e1e24');
    
    // Adjust size to container
    const width = 800;
    const height = 450;
    const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    
    renderer.setSize(width, height);
    canvasRef.current.innerHTML = '';
    canvasRef.current.appendChild(renderer.domElement);

    // Add Grid Helper
    const gridHelper = new THREE.GridHelper(10, 10, 0x4f46e5, 0x475569);
    scene.add(gridHelper);

    // Initial 3D Mesh
    const geometry = new THREE.BoxGeometry(1.5, 1.5, 1.5);
    
    // We'll create a fancy material
    const material = new THREE.MeshStandardMaterial({ 
      color: 0x22d3ee,
      wireframe: true,
      emissive: 0x0f172a,
      roughness: 0.2,
      metalness: 0.8
    });
    
    const cube = new THREE.Mesh(geometry, material);
    scene.add(cube);

    // Adding some lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    
    const pointLight = new THREE.PointLight(0xffffff, 1);
    pointLight.position.set(5, 5, 5);
    scene.add(pointLight);

    camera.position.z = 4;
    camera.position.y = 1;
    camera.lookAt(0, 0, 0);

    // Animation Loop
    let animationId: number;
    const animate = function () {
      animationId = requestAnimationFrame(animate);
      cube.rotation.x += 0.01;
      cube.rotation.y += 0.01;
      renderer.render(scene, camera);
    };

    animate();

    return () => {
      cancelAnimationFrame(animationId);
      if (canvasRef.current) {
        canvasRef.current.innerHTML = '';
      }
      renderer.dispose();
      geometry.dispose();
      material.dispose();
    };
  }, [activeTab]);

  return (
    <div className="flex flex-col h-full bg-[#1e1e24] text-slate-200">
      <div className="flex items-center justify-between p-2 bg-[#2d2d36] border-b border-white/5">
        <div className="flex gap-2">
          <button 
            onClick={() => setActiveTab('2d')}
            className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${activeTab === '2d' ? 'bg-indigo-500 text-white' : 'bg-transparent text-slate-400 hover:text-white'} flex items-center gap-2`}
          >
            <Pencil className="w-4 h-4" /> dPaint 2D
          </button>
          <button 
            onClick={() => setActiveTab('3d')}
            className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${activeTab === '3d' ? 'bg-indigo-500 text-white' : 'bg-transparent text-slate-400 hover:text-white'} flex items-center gap-2`}
          >
            <Box className="w-4 h-4" /> dPaint 3D Mesh
          </button>
        </div>
        <div className="flex gap-2 text-slate-400">
           <button className="p-1.5 hover:bg-white/10 rounded transition-colors" title="Export">
             <Download className="w-4 h-4" />
           </button>
           <button className="p-1.5 hover:bg-white/10 rounded transition-colors" title="Clear">
             <Trash className="w-4 h-4" />
           </button>
        </div>
      </div>
      
      <div className="flex flex-1 overflow-hidden">
        {/* Toolbar */}
        <div className="w-16 bg-[#25252d] border-r border-white/5 flex flex-col items-center py-4 gap-4">
          <button className="p-2 bg-white/10 text-white rounded-md hover:bg-white/20 transition"><Pencil className="w-5 h-5" /></button>
          <button className="p-2 text-slate-400 hover:bg-white/10 hover:text-white rounded-md transition"><Square className="w-5 h-5" /></button>
          <button className="p-2 text-slate-400 hover:bg-white/10 hover:text-white rounded-md transition"><Circle className="w-5 h-5" /></button>
          {activeTab === '3d' && (
            <>
              <div className="w-8 h-px bg-white/10 my-1" />
              <button className="p-2 text-indigo-400 bg-indigo-500/10 hover:bg-indigo-500/20 rounded-md transition" title="Add Cube"><Box className="w-5 h-5" /></button>
              <button className="p-2 text-slate-400 hover:bg-white/10 hover:text-white rounded-md transition" title="Rotate"><Rotate3D className="w-5 h-5" /></button>
            </>
          )}
        </div>
        
        {/* Canvas Area */}
        <div className="flex-1 bg-white/5 p-4 flex items-center justify-center relative shadow-inner">
          <div className="w-[800px] max-w-full aspect-video bg-white shadow-2xl relative border border-white/10 overflow-hidden flex items-center justify-center" ref={activeTab === '3d' ? canvasRef : null}>
            {activeTab === '2d' && (
              <>
                <div className="absolute inset-0 pointer-events-none opacity-20" style={{ backgroundImage: 'radial-gradient(#000 1px, transparent 1px)', backgroundSize: '20px 20px' }}></div>
                <div className="text-slate-400 font-mono text-sm pointer-events-none opacity-50 flex items-center gap-2">
                   <Pencil className="w-4 h-4" /> 2D Canvas Ready
                </div>
              </>
            )}
          </div>
          {activeTab === '3d' && (
             <div className="absolute top-6 right-6 text-xs font-mono text-white/50 pointer-events-none">dMesh Renderer Active (Three.js)</div>
          )}
        </div>
      </div>
    </div>
  );
}
