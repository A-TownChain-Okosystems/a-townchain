import React, { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { Bot, BrainCircuit, Globe, Settings, HardDrive, Folder, Activity, LayoutGrid, Terminal, Database, Code, Map, Shield } from 'lucide-react';

const appsList = [
  { id: 'overview', name: "Dashboard", Icon: LayoutGrid, color: '#4ade80' },
  { id: 'atown_os', name: "ATC-OS", Icon: Bot, color: '#818cf8' },
  { id: 'ai_kernel', name: "AI Kernel", Icon: BrainCircuit, color: '#c084fc' },
  { id: 'atown_browser', name: "GateToHell", Icon: Globe, color: '#f472b6' },
  { id: 'settings', name: "Einstellungen", Icon: Settings, color: '#94a3b8' },
  { id: 'storage_mgr', name: "Laufwerke", Icon: HardDrive, color: '#60a5fa' },
  { id: 'file_mgr', name: "Explorer", Icon: Folder, color: '#fbbf24' },
  { id: 'task_mgr', name: "System", Icon: Activity, color: '#38bdf8' },
  { id: 'terminal', name: "Terminal", Icon: Terminal, color: '#a3e635' },
  { id: 'datalake', name: "Data Lake", Icon: Database, color: '#2dd4bf' },
  { id: 'source_code', name: "System Code", Icon: Code, color: '#f87171' },
  { id: 'project_hub', name: "Project Hub", Icon: Map, color: '#fb923c' },
  { id: 'zk_proofs', name: "ZK Subsystem", Icon: Shield, color: '#e879f9' }
];

export function AppGlobeView() {
  const mountRef = useRef<HTMLDivElement>(null);
  const appRefs = useRef<(HTMLDivElement | null)[]>([]);

  useEffect(() => {
    if (!mountRef.current) return;

    const width = mountRef.current.clientWidth;
    const height = mountRef.current.clientHeight;

    // --- GRUNDLAGEN ---
    const scene = new THREE.Scene();
    scene.background = null; // Transparent background

    const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    camera.position.set(0, 0, 8);

    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setClearColor(0x000000, 0); // Transparent clear color
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    mountRef.current.appendChild(renderer.domElement);

    // --- INTERAKTION ---
    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;
    controls.minDistance = 4;
    controls.maxDistance = 20;

    // --- BELEUCHTUNG ---
    const centralLight = new THREE.PointLight(0xffffff, 2, 20);
    scene.add(centralLight);
    const ambientLight = new THREE.AmbientLight(0x404040, 1);
    scene.add(ambientLight);

    // --- HINTERGRUND-ELEMENTE (Sterne) ---
    const starsGeometry = new THREE.BufferGeometry();
    const starsPositions = [];
    for (let i = 0; i < 2000; i++) {
        starsPositions.push((Math.random() - 0.5) * 100);
        starsPositions.push((Math.random() - 0.5) * 100);
        starsPositions.push((Math.random() - 0.5) * 100);
    }
    starsGeometry.setAttribute('position', new THREE.Float32BufferAttribute(starsPositions, 3));
    const starsMaterial = new THREE.PointsMaterial({ color: 0x888888, size: 0.1 });
    const stars = new THREE.Points(starsGeometry, starsMaterial);
    scene.add(stars);

    // --- APP-MENÜ-GLOBUS ---
    const appsGroup = new THREE.Group();
    scene.add(appsGroup);

    // Globe Mesh
    const textureLoader = new THREE.TextureLoader();
    const globeGeometry = new THREE.SphereGeometry(3.8, 64, 64);
    const globeMaterial = new THREE.MeshStandardMaterial({
      color: 0x2233ff,
      emissive: 0x001133,
      roughness: 0.6,
      metalness: 0.2,
      map: textureLoader.load('https://unpkg.com/three-globe/example/img/earth-blue-marble.jpg'),
      transparent: true,
      opacity: 0.95
    });
    const globeMesh = new THREE.Mesh(globeGeometry, globeMaterial);
    appsGroup.add(globeMesh);

    // Atmosphere Mesh
    const atmosphereGeometry = new THREE.SphereGeometry(4.0, 32, 32);
    const atmosphereMaterial = new THREE.MeshPhongMaterial({
      color: 0x4f46e5,
      transparent: true,
      opacity: 0.15,
      side: THREE.BackSide,
      blending: THREE.AdditiveBlending,
    });
    const atmosphereMesh = new THREE.Mesh(atmosphereGeometry, atmosphereMaterial);
    appsGroup.add(atmosphereMesh);

    const numApps = appsList.length;
    const radius = 4.2; // Größerer Radius für besseren Abstand über der Kugel

    const appPoints: THREE.Mesh[] = [];

    appsList.forEach((_, i) => {
        const phi = Math.acos(-1 + (2 * i) / numApps);
        const theta = Math.sqrt(numApps * Math.PI) * phi;

        // Unsichtbares Mesh für die Positionsberechnung
        const geometry = new THREE.BoxGeometry(0.1, 0.1, 0.1);
        const material = new THREE.MeshBasicMaterial({ visible: false });
        const dummyMesh = new THREE.Mesh(geometry, material);

        dummyMesh.position.setFromSphericalCoords(radius, phi, theta);
        appsGroup.add(dummyMesh);
        appPoints.push(dummyMesh);
    });

    const handleResize = () => {
      if (!mountRef.current) return;
      const newWidth = mountRef.current.clientWidth;
      const newHeight = mountRef.current.clientHeight;
      camera.aspect = newWidth / newHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(newWidth, newHeight);
    };
    window.addEventListener('resize', handleResize);

    let animationFrameId: number;
    const vector = new THREE.Vector3();

    const animate = () => {
      animationFrameId = requestAnimationFrame(animate);

      // Leichte automatische Rotation, wenn nicht interagiert wird
      appsGroup.rotation.y += 0.003;

      controls.update();
      renderer.render(scene, camera);

      // Aktualisierung der HTML 2D-Overlays
      if (mountRef.current) {
        const w = mountRef.current.clientWidth;
        const h = mountRef.current.clientHeight;
        const halfW = w / 2;
        const halfH = h / 2;

        const cameraPos = new THREE.Vector3();
        camera.getWorldPosition(cameraPos);
        const globeCenter = new THREE.Vector3();
        appsGroup.getWorldPosition(globeCenter);
        const viewVector = new THREE.Vector3().subVectors(cameraPos, globeCenter).normalize();

        appPoints.forEach((mesh, i) => {
          const el = appRefs.current[i];
          if (!el) return;

          vector.setFromMatrixPosition(mesh.matrixWorld);
          
          const worldPos = new THREE.Vector3();
          mesh.getWorldPosition(worldPos);
          const normalVector = new THREE.Vector3().subVectors(worldPos, globeCenter).normalize();
          const dot = viewVector.dot(normalVector);

          vector.project(camera);

          // Nur anzeigen, wenn das Element auf der vorderen Halbkugel ist
          if (vector.z < 1 && dot > -0.2) {
            const x = (vector.x * halfW) + halfW;
            const y = -(vector.y * halfH) + halfH;
            
            const zIndex = Math.round((1 - vector.z) * 100);
            const scale = Math.max(0.4, 1 - vector.z * 0.5) * (dot > 0 ? 1 : (1 + dot * 2));
            const opacity = Math.max(0, Math.min(1, (dot + 0.2) * 2));

            el.style.display = 'flex';
            el.style.transform = `translate(-50%, -50%) translate(${x}px, ${y}px) scale(${scale})`;
            el.style.zIndex = zIndex.toString();
            el.style.opacity = opacity.toString();
          } else {
            el.style.display = 'none';
          }
        });
      }
    };
    animate();

    return () => {
      window.removeEventListener('resize', handleResize);
      cancelAnimationFrame(animationFrameId);
      if (mountRef.current) {
        mountRef.current.removeChild(renderer.domElement);
      }
      renderer.dispose();
    };
  }, []);

  const handleAppClick = (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    window.dispatchEvent(new CustomEvent('open-window', { detail: { id } }));
  };

  return (
    <div className="absolute inset-0 pointer-events-none flex flex-col z-0">
      <div ref={mountRef} className="absolute inset-0 pointer-events-auto cursor-grab active:cursor-grabbing" />
      
      {/* 2D HTML Overlays for Apps */}
      <div className="absolute inset-0 pointer-events-none">
        {appsList.map((app, index) => (
          <div
            key={app.id}
            ref={el => { appRefs.current[index] = el; }}
            className="absolute top-0 left-0 flex flex-col items-center justify-center p-2 group transition-colors pointer-events-auto cursor-pointer"
            onClick={(e) => handleAppClick(app.id, e)}
            style={{ transformOrigin: 'center center' }}
          >
            <div 
              className="w-16 h-16 rounded-2xl bg-[#090b14]/80 border-2 backdrop-blur-md flex items-center justify-center mb-2 shadow-[0_0_15px_rgba(0,0,0,0.5)] transition-transform group-hover:scale-110"
              style={{ borderColor: app.color, boxShadow: `0 0 20px ${app.color}40` }}
            >
              <app.Icon size={32} style={{ color: app.color }} />
            </div>
            <span className="text-white font-bold text-sm text-center drop-shadow-[0_2px_2px_rgba(0,0,0,1)] bg-black/60 border border-white/10 px-3 py-1 rounded-lg backdrop-blur-sm whitespace-nowrap">
              {app.name}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
