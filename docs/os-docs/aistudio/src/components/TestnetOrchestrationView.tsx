import React, { useState } from 'react';
import { Server, Network, Terminal, Settings, Copy, Check, Play, Shield, Database, Cpu } from 'lucide-react';

const DOCKER_COMPOSE_YML = `version: '3.8'
# ==============================================================================
# A-TownChain Testnet Orchestration (Multi-Node Cluster)
# Simuliert ein dezentrales Mainnet-Umfeld für Entwickler
# ==============================================================================
services:

  # 1. Der Bootnode (Kademlia DHT Ankerpunkt)
  atc-bootnode:
    image: shivacoredev/atc-core-node:latest
    container_name: atc_testnet_bootnode
    command: ["--mode", "seed", "--port", "30333", "--init-genesis"]
    networks:
      - atc-testnet
    ports:
      - "30333:30333"

  # 2. Validator A (Konsens & PoAI)
  atc-validator-alpha:
    image: shivacoredev/atc-core-node:latest
    container_name: atc_testnet_val_alpha
    depends_on:
      - atc-bootnode
    command: ["--mode", "validator", "--bootnode", "atc-bootnode:30333", "--stake", "50000", "--enable-poai", "true"]
    networks:
      - atc-testnet

  # 3. Validator B (Konsens & ATC-Shield)
  atc-validator-beta:
    image: shivacoredev/atc-core-node:latest
    container_name: atc_testnet_val_beta
    depends_on:
      - atc-bootnode
    command: ["--mode", "validator", "--bootnode", "atc-bootnode:30333", "--stake", "25000", "--enable-poai", "true", "--enable-atc-shield", "true"]
    networks:
      - atc-testnet

  # 4. Der RPC-Endpunkt für GlobusOS (A-Portal)
  atc-rpc-gateway:
    image: shivacoredev/atc-rpc-gateway:latest
    container_name: atc_testnet_rpc
    depends_on:
      - atc-bootnode
    command: ["--connect", "atc-bootnode:50051", "--listen-port", "8080"]
    networks:
      - atc-testnet
    ports:
      - "8080:8080" # GlobusOS verbindet sich hierhin

networks:
  atc-testnet:
    driver: bridge`;

export function TestnetOrchestrationView() {
  const [copied, setCopied] = useState(false);
  const [clusterState, setClusterState] = useState<'stopped' | 'starting' | 'running'>('stopped');

  const copyToClipboard = () => {
    navigator.clipboard.writeText(DOCKER_COMPOSE_YML);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const simulateDeploy = () => {
    if (clusterState === 'running') {
      setClusterState('stopped');
    } else {
      setClusterState('starting');
      setTimeout(() => setClusterState('running'), 2000);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#050B14] border border-white/5 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center gap-3 px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <Server className="w-5 h-5 text-emerald-400" />
        <div>
          <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">Testnet Orchestrator</h2>
          <p className="text-xs text-slate-400 mt-0.5">Docker-Compose Multi-Node Cluster Configuration</p>
        </div>
      </div>
      
      <div className="flex-1 flex overflow-hidden">
        {/* Left Side: Docker-Compose Code */}
        <div className="w-1/2 p-6 overflow-y-auto custom-scrollbar border-r border-white/5 relative bg-[#020408]">
            <div className="absolute top-6 right-6 flex gap-2">
                <button 
                  onClick={copyToClipboard}
                  className="p-2 bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white rounded border border-white/10 transition-colors"
                  title="Copy YAML"
                >
                    {copied ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                </button>
            </div>
            
            <h3 className="font-mono text-xs text-slate-400 uppercase flex items-center gap-2 mb-4">
                <Terminal className="w-4 h-4 text-slate-500" /> docker-compose.yml
            </h3>
            
            <pre className="font-mono text-[10px] text-emerald-400/80 whitespace-pre-wrap leading-relaxed">
                {DOCKER_COMPOSE_YML}
            </pre>
        </div>

        {/* Right Side: Visualizer */}
        <div className="w-1/2 bg-[#050B14] flex flex-col">
            <div className="p-6 border-b border-white/5 flex items-center justify-between">
                <div className="flex items-center gap-2 text-sm font-mono text-slate-300">
                    <Network className="w-4 h-4 text-indigo-400" /> Cluster Topology
                </div>
                <button 
                    onClick={simulateDeploy}
                    className={`px-4 py-2 font-mono text-[10px] uppercase tracking-wider rounded border transition-all flex items-center gap-2 ${
                        clusterState === 'running' 
                        ? 'bg-rose-500/10 text-rose-400 border-rose-500/30 hover:bg-rose-500/20'
                        : 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30 hover:bg-emerald-500/20'
                    }`}
                >
                    {clusterState === 'running' ? 'Stop Cluster' : clusterState === 'starting' ? 'Bootstap...' : <><Play className="w-3.5 h-3.5" /> Start Cluster</>}
                </button>
            </div>
            
            <div className="flex-1 p-8 overflow-y-auto custom-scrollbar flex flex-col items-center justify-center relative">
               {/* Connections */}
               <div className="absolute inset-0 z-0 opacity-20 pointer-events-none">
                  <svg className="w-full h-full">
                     <line x1="50%" y1="20%" x2="30%" y2="50%" stroke="#818cf8" strokeWidth="2" strokeDasharray="4" className={clusterState === 'running' ? "animate-pulse" : ""} />
                     <line x1="50%" y1="20%" x2="70%" y2="50%" stroke="#818cf8" strokeWidth="2" strokeDasharray="4" className={clusterState === 'running' ? "animate-pulse" : ""} />
                     <line x1="50%" y1="20%" x2="50%" y2="80%" stroke="#818cf8" strokeWidth="2" strokeDasharray="4" className={clusterState === 'running' ? "animate-pulse" : ""} />
                  </svg>
               </div>
               
               {/* Bootnode */}
               <div className={`z-10 bg-slate-900 border ${clusterState === 'running' ? 'border-indigo-500/50 shadow-[0_0_20px_rgba(99,102,241,0.2)]' : 'border-white/10'} p-4 rounded-xl w-64 mb-16 text-center transition-all duration-500`}>
                   <div className="flex justify-center mb-2">
                       <Database className={`w-8 h-8 ${clusterState === 'running' ? 'text-indigo-400' : 'text-slate-600'}`} />
                   </div>
                   <h4 className="font-mono text-sm text-white mb-1">atc-bootnode</h4>
                   <p className="text-[10px] text-slate-500 font-mono">Port: 30333 | Mode: seed</p>
               </div>

               <div className="flex justify-between w-full max-w-lg z-10 relative">
                  {/* Validator Alpha */}
                  <div className={`bg-slate-900 border ${clusterState === 'running' ? 'border-emerald-500/50' : 'border-white/10'} p-4 rounded-xl w-48 text-center transition-all duration-500 delay-100`}>
                      <div className="flex justify-center mb-2">
                          <Cpu className={`w-6 h-6 ${clusterState === 'running' ? 'text-emerald-400' : 'text-slate-600'}`} />
                      </div>
                      <h4 className="font-mono text-xs text-white mb-1">Validator Alpha</h4>
                      <p className="text-[10px] text-slate-500 font-mono">Stake: 50k | PoAI</p>
                  </div>

                  {/* RPC Gateway */}
                 <div className={`absolute left-1/2 top-16 -translate-x-1/2 bg-slate-900 border ${clusterState === 'running' ? 'border-blue-500/50' : 'border-white/10'} p-4 rounded-xl w-48 text-center transition-all duration-500 delay-300`}>
                      <div className="flex justify-center mb-2">
                          <Network className={`w-6 h-6 ${clusterState === 'running' ? 'text-blue-400' : 'text-slate-600'}`} />
                      </div>
                      <h4 className="font-mono text-xs text-white mb-1">RPC Gateway</h4>
                      <p className="text-[10px] text-slate-500 font-mono">Port: 8080</p>
                  </div>

                  {/* Validator Beta */}
                  <div className={`bg-slate-900 border ${clusterState === 'running' ? 'border-purple-500/50' : 'border-white/10'} p-4 rounded-xl w-48 text-center transition-all duration-500 delay-200`}>
                      <div className="flex justify-center mb-2">
                          <Shield className={`w-6 h-6 ${clusterState === 'running' ? 'text-purple-400' : 'text-slate-600'}`} />
                      </div>
                      <h4 className="font-mono text-xs text-white mb-1">Validator Beta</h4>
                      <p className="text-[10px] text-slate-500 font-mono">Stake: 25k | Shield</p>
                  </div>
               </div>
            </div>
        </div>
      </div>
    </div>
  );
}
