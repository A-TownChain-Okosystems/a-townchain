import React, { useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Folder, FolderOpen, File, FileCode, FileJson, FileText, FileSearch, Database, Shield, Hexagon, Terminal, Box, Boxes, Lock, Cpu, Brain, Network, FileDigit, Copy, Check } from 'lucide-react';

interface FileNode {
  id: string;
  name: string;
  type: 'folder' | 'file';
  extension?: string;
  description?: string;
  children?: FileNode[];
  isOpen?: boolean;
}

export const INITIAL_STRUCTURE: FileNode[] = [
  {
    id: 'atc-ecosystem',
    name: 'ATC-Ecosystem-Root',
    type: 'folder',
    description: 'Master Production Architecture (30 Layers)',
    isOpen: true,
    children: [
      {
        id: 'layer-01-kernel',
        name: '01_Kernel',
        type: 'folder',
        description: 'Hardware Foundation & ATC-OS Kernel',
        children: [
          { id: 'boot.atc', name: 'boot.atc', type: 'file', extension: 'atc', description: 'Primary Executable for Boot System' },
          { id: 'mmu.atc', name: 'mmu.atc', type: 'file', extension: 'atc', description: 'Memory Management Unit Executable' },
          { id: 'scheduler.atc', name: 'scheduler.atc', type: 'file', extension: 'atc', description: 'Core Multitasking Scheduler' }
        ]
      },
      {
        id: 'layer-02-bridge',
        name: '02_Bridge_Compatibility',
        type: 'folder',
        description: 'Bridge Compatibility Layer (BCL)',
        children: [
          { id: 'core-adapter.atb', name: 'core-adapter.atb', type: 'file', extension: 'atb', description: 'Interface Normalization Bridge' },
          { id: 'schema-router.atb', name: 'schema-router.atb', type: 'file', extension: 'atb', description: 'Routing Engine Compatibility Module' }
        ]
      },
      {
        id: 'layer-03-runtime',
        name: '03_Runtime_Engine',
        type: 'folder',
        description: 'Execution Engine & Dispatchers',
        children: [
          { id: 'dispatcher.aes', name: 'dispatcher.aes', type: 'file', extension: 'aes', description: 'Primary Execution Unit' },
          { id: 'resource-mgr.aes', name: 'resource-mgr.aes', type: 'file', extension: 'aes', description: 'Runtime Resource Manager' }
        ]
      },
      {
        id: 'layer-04-vm',
        name: '04_Virtual_Machine',
        type: 'folder',
        description: 'ATC VM - Deterministic Bytegraph Engine',
        children: [
          { id: 'bytegraph.atvm', name: 'bytegraph.atvm', type: 'file', extension: 'atvm', description: 'Core VM Module Engine' },
          { id: 'sandbox.atvm', name: 'sandbox.atvm', type: 'file', extension: 'atvm', description: 'Isolated Execution Environment' }
        ]
      },
      {
        id: 'layer-05-smartcontracts',
        name: '05_Smart_Contracts',
        type: 'folder',
        description: 'Native ATC Contracts (ATS Standards)',
        children: [
          { id: 'ATCAsset-gaming.atpkg', name: 'ATCAsset-gaming.atpkg', type: 'file', extension: 'atpkg', description: 'ATC-9000 NFT Standard Logic' },
          { id: 'governance-dao.atpkg', name: 'governance-dao.atpkg', type: 'file', extension: 'atpkg', description: 'ATS-Gov On-Chain Deployment' }
        ]
      },
      {
        id: 'layer-06-state',
        name: '06_Storage_State',
        type: 'folder',
        description: 'Decentralized State & DB Layers',
        children: [
          { id: 'global-tree.ats', name: 'global-tree.ats', type: 'file', extension: 'ats', description: 'Merkle-Patricia-Trie Root State' },
          { id: 'history.atsdb', name: 'history.atsdb', type: 'file', extension: 'atsdb', description: 'Journaling DB / Cold Storage' }
        ]
      },
      {
        id: 'layer-07-network',
        name: '07_Network_ATP',
        type: 'folder',
        description: 'ATC Transport Protocol (ATP) & P2P',
        children: [
          { id: 'gossip.atn', name: 'gossip.atn', type: 'file', extension: 'atn', description: 'Network Routing & Gossip Protocol' },
          { id: 'discovery.atn', name: 'discovery.atn', type: 'file', extension: 'atn', description: 'Peer Discovery Mesh Definition' }
        ]
      },
      {
        id: 'layer-08-ai',
        name: '08_Autonomous_AI',
        type: 'folder',
        description: 'ATS Autonomous AI Engine & Governance',
        children: [
          { id: 'aurora-orchestrator.ata', name: 'aurora-orchestrator.ata', type: 'file', extension: 'ata', description: 'AI Agent Coordinator' },
          { id: 'poai-verifier.ata', name: 'poai-verifier.ata', type: 'file', extension: 'ata', description: 'Proof of AI Verification Agent' },
          { id: 'gpu-infer.atg', name: 'gpu-infer.atg', type: 'file', extension: 'atg', description: 'GPU Shader Matrix for LLMs' }
        ]
      },
      {
        id: 'layer-09-identity',
        name: '09_Identity_Wallet',
        type: 'folder',
        description: 'DID, KYC and MPC Key Management',
        children: [
          { id: 'treasury-multi.atk', name: 'treasury-multi.atk', type: 'file', extension: 'atk', description: 'Treasury Wallet Keys (MPC/Multi-sig)' },
          { id: 'system-identity.atk', name: 'system-identity.atk', type: 'file', extension: 'atk', description: 'Core Node ZKP Identity' }
        ]
      },
      {
        id: 'layer-10-ops',
        name: '10_Global_Ops',
        type: 'folder',
        description: 'Global Operations Control Plane',
        children: [
          { id: 'dashboard.config.json', name: 'dashboard.config.json', type: 'file', extension: 'json', description: 'Global UI Configuration' },
          { id: 'system.metrics.prom', name: 'system.metrics.prom', type: 'file', extension: 'prom', description: 'Prometheus Metric Exporters' },
          { id: 'kubernetes-mesh.yaml', name: 'kubernetes-mesh.yaml', type: 'file', extension: 'yaml', description: 'Layer 25 K8s Service Mesh' }
        ]
      },
      {
        id: 'layer-11-randd',
        name: '11_Advanced_R_and_D',
        type: 'folder',
        description: 'Zukünftige & Erweiterte Komponenten (R&D)',
        children: [
          { id: 'hardware-enclave.atb', name: 'hardware-enclave.atb', type: 'file', extension: 'atb', description: 'TEE Bridge for Secure Exection' },
          { id: 'quantum-archival.atsdb', name: 'quantum-archival.atsdb', type: 'file', extension: 'atsdb', description: 'Post-Quantum Cold Storage Journal' },
          { id: 'cross-rollup.atn', name: 'cross-rollup.atn', type: 'file', extension: 'atn', description: 'Data Availability Layer routing' },
          { id: 'meta-reality.atn', name: 'meta-reality.atn', type: 'file', extension: 'atn', description: 'Spatial Computing Network Gateway' }
        ]
      }
    ]
  }
];

export function StructureView() {
  const [structure, setStructure] = useState<FileNode[]>(INITIAL_STRUCTURE);
  const [selectedFile, setSelectedFile] = useState<FileNode | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  React.useEffect(() => {
    const handleDeepLink = (e: any) => {
      const id = e.detail.id;
      // find node
      let foundNode: FileNode | null = null;
      const expandParents = (nodes: FileNode[]): boolean => {
        let found = false;
        for (const node of nodes) {
          if (node.id === id) {
            foundNode = node;
            found = true;
          } else if (node.children && expandParents(node.children)) {
            node.isOpen = true;
            found = true;
          }
        }
        return found;
      };
      
      setStructure(prev => {
        const next = JSON.parse(JSON.stringify(prev));
        expandParents(next);
        return next;
      });
      
      setTimeout(() => {
        if (foundNode) {
          setSelectedFile(foundNode);
          const el = document.getElementById(`structure-node-${id}`);
          if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
      }, 100);
    };
    window.addEventListener('ATC_DEEP_LINK_STRUCTURE', handleDeepLink);
    return () => window.removeEventListener('ATC_DEEP_LINK_STRUCTURE', handleDeepLink);
  }, []);

  const toggleFolder = (nodeId: string, currentNodes: FileNode[]): FileNode[] => {
    return currentNodes.map(node => {
      if (node.id === nodeId) {
        return { ...node, isOpen: !node.isOpen };
      }
      if (node.children) {
        return { ...node, children: toggleFolder(nodeId, node.children) };
      }
      return node;
    });
  };

  const handleNodeClick = (node: FileNode) => {
    if (node.type === 'folder') {
      setStructure(prev => toggleFolder(node.id, prev));
    } else {
      setSelectedFile(node);
    }
  };

  const getFileIcon = (extension?: string) => {
    switch (extension) {
      case 'atc':
      case 'aes':
        return <Terminal className="w-4 h-4 text-emerald-400" />;
      case 'atvm':
        return <Cpu className="w-4 h-4 text-indigo-400" />;
      case 'ata':
        return <Brain className="w-4 h-4 text-pink-400" />;
      case 'ats':
      case 'atsdb':
        return <Database className="w-4 h-4 text-atc-cyan" />;
      case 'atn':
        return <Network className="w-4 h-4 text-blue-400" />;
      case 'atk':
        return <Lock className="w-4 h-4 text-orange-400" />;
      case 'atb':
        return <Hexagon className="w-4 h-4 text-purple-400" />;
      case 'atpkg':
        return <Boxes className="w-4 h-4 text-slate-300" />;
      case 'atg':
        return <Box className="w-4 h-4 text-green-400" />;
      case 'json':
      case 'yaml':
      case 'prom':
        return <FileJson className="w-4 h-4 text-slate-400" />;
      default:
        return <FileDigit className="w-4 h-4 text-slate-400" />;
    }
  };

  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number, y: number, node: FileNode } | null>(null);

  React.useEffect(() => {
    const handleGlobalClick = () => setContextMenu(null);
    document.addEventListener('mousedown', handleGlobalClick);
    return () => document.removeEventListener('mousedown', handleGlobalClick);
  }, []);

  const renderFileNodeItem = (node: FileNode, level: number, hasMatchingChild: boolean, isSelected: boolean) => {
    const handleCopyLink = (e: React.MouseEvent, node: FileNode) => {
      e.stopPropagation();
      const copyText = `[ATC-OS Module Path]: /${node.name}\n${node.description ? `[Description]: ${node.description}\n` : ''}`;
      navigator.clipboard.writeText(copyText);
      setCopiedId(node.id);
      setTimeout(() => setCopiedId(null), 2000);
    };

    return (
      <div 
        key={node.id} 
        id={`structure-node-${node.id}`} 
        className="flex flex-col group relative"
        onContextMenu={(e) => {
          e.preventDefault();
          e.stopPropagation();
          setContextMenu({ x: e.clientX, y: e.clientY, node });
        }}
      >
        <div 
          onClick={() => handleNodeClick(node)}
          className={`flex items-center justify-between gap-2 py-1.5 px-2 rounded-lg cursor-pointer transition-colors ${
            isSelected 
              ? 'bg-atc-purple/20 text-atc-purple' 
              : 'hover:bg-atc-bg/80 text-slate-300 hover:text-white'
          }`}
          style={{ paddingLeft: `${level * 1.5 + 0.5}rem` }}
        >
          <div className="flex items-center gap-2">
            {node.type === 'folder' ? (
              node.isOpen || hasMatchingChild ? (
                <FolderOpen className="w-4 h-4 text-atc-purple fill-atc-purple/20" />
              ) : (
                <Folder className="w-4 h-4 text-atc-purple fill-atc-purple/20" />
              )
            ) : (
              getFileIcon(node.extension)
            )}
            <span className={`text-sm font-mono ${node.type === 'folder' ? 'font-semibold tracking-wide' : ''}`}>
              {node.name}
            </span>
          </div>

          {node.type === 'file' && (
            <button 
              onClick={(e) => handleCopyLink(e, node)}
              className="opacity-0 group-hover:opacity-100 p-1 text-slate-400 hover:text-atc-cyan transition-all"
              title="Copy Module Path"
            >
              {copiedId === node.id ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
            </button>
          )}
        </div>
        
        <AnimatePresence>
          {node.type === 'folder' && (node.isOpen || hasMatchingChild) && node.children && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: 'auto', opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              transition={{ duration: 0.2 }}
              className="overflow-hidden"
            >
              {renderTree(node.children, level + 1)}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    );
  };

  const renderTree = (nodes: FileNode[], level = 0) => {
    return nodes.map(node => {
      const isMatch = searchQuery === '' || node.name.toLowerCase().includes(searchQuery.toLowerCase());
      const hasMatchingChild = node.children && hasMatch(node.children, searchQuery);
      
      if (!isMatch && !hasMatchingChild) return null;

      return renderFileNodeItem(node, level, hasMatchingChild, selectedFile?.id === node.id);
    });
  };

  const hasMatch = (nodes: FileNode[], query: string): boolean => {
    if (query === '') return true;
    for (const node of nodes) {
      if (node.name.toLowerCase().includes(query.toLowerCase())) return true;
      if (node.children && hasMatch(node.children, query)) return true;
    }
    return false;
  };

  return (
    <div className="flex flex-col gap-6 mt-6 pb-12 w-full max-w-6xl mx-auto h-[800px]">
      <div className="flex items-center justify-between gap-4 mb-2">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-atc-purple/10 border border-atc-purple/20 flex items-center justify-center text-atc-purple shadow-[0_0_15px_rgba(162,89,255,0.2)]">
            <FolderOpen className="w-5 h-5" />
          </div>
          <div>
            <h2 className="text-2xl font-semibold tracking-tight text-white">Dateistruktur</h2>
            <p className="text-sm font-light text-slate-400">Architektur-Organisation von ATC-OS / A-TownChain</p>
          </div>
        </div>
        
        <div className="relative">
          <FileSearch className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
          <input 
            type="text" 
            placeholder="Search files..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-64 bg-[#090b14]/80 border border-atc-border/80 focus:border-atc-cyan/50 focus:ring-1 focus:ring-atc-cyan/50 rounded-lg py-2 pl-9 pr-4 text-sm text-slate-200 outline-none transition-all shadow-[0_4px_20px_rgba(0,0,0,0.5)] placeholder:text-slate-500 font-mono"
          />
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl shadow-2xl relative">
        <div className="absolute top-0 right-0 w-64 h-64 bg-atc-cyan/5 rounded-full blur-[100px] pointer-events-none" />
        
        {/* Architecture Explorer */}
        <div className="w-1/2 md:w-1/3 lg:w-1/4 border-r border-atc-border/50 bg-[#090b14]/60 flex flex-col p-4 overflow-y-auto custom-scrollbar relative z-10">
          <h3 className="text-[10px] font-mono text-slate-500 uppercase tracking-widest pl-2 mb-4 border-b border-atc-border/50 pb-2">Explorer</h3>
          <div className="flex flex-col gap-0.5">
            {renderTree(structure)}
          </div>
        </div>

        {/* File Detail/Content View */}
        <div className="flex-1 overflow-y-auto custom-scrollbar bg-[#0a0f1d]/50 relative z-10 flex flex-col">
          {selectedFile ? (
            <div className="flex flex-col h-full">
              <div className="flex items-center gap-3 px-6 py-4 border-b border-atc-border/50 bg-[#090b14]/80">
                {getFileIcon(selectedFile.extension)}
                <span className="text-sm font-mono text-slate-200">{selectedFile.name}</span>
                <span className="ml-auto text-[10px] uppercase font-mono tracking-widest text-slate-500 font-bold bg-[#090b14] px-2 py-1 rounded border border-atc-border/50">
                  {selectedFile.extension || 'FILE'}
                </span>
              </div>
              
              <div className="p-8 flex-1">
                <div className="max-w-2xl mx-auto">
                  <div className="p-6 rounded-xl border border-atc-border/50 bg-[#090b14]/80 shadow-[0_8px_30px_rgba(0,0,0,0.4)] relative overflow-hidden group">
                    <div className="absolute top-0 right-0 w-32 h-32 bg-atc-purple/5 rounded-full blur-[40px] pointer-events-none group-hover:bg-atc-purple/10 transition-colors" />
                    
                    <h3 className="text-xl font-medium text-white tracking-tight mb-2 relative z-10">{selectedFile.name}</h3>
                    <p className="text-slate-400 text-sm leading-relaxed relative z-10 mb-6">
                      {selectedFile.description || 'No description available for this file.'}
                    </p>
                    
                    <div className="space-y-4 relative z-10">
                      <div className="flex flex-col gap-1.5">
                        <span className="text-[10px] font-mono uppercase tracking-widest text-slate-500 font-bold">Metadata</span>
                        <div className="grid grid-cols-2 gap-4 p-4 rounded-lg border border-atc-border/50 bg-[#060a16]">
                           <div>
                             <span className="block text-[10px] text-slate-500 uppercase tracking-wider mb-1">Type</span>
                             <span className="text-sm text-slate-300">{selectedFile.extension === 'ts' || selectedFile.extension === 'tsx' ? 'atc-lang Source' : selectedFile.extension === 'sol' ? 'atc-lang Smart Contract' : 'Configuration / Document'}</span>
                           </div>
                           <div>
                             <span className="block text-[10px] text-slate-500 uppercase tracking-wider mb-1">Status</span>
                             <span className="inline-flex items-center gap-1.5 text-xs text-emerald-400">
                               <div className="w-1.5 h-1.5 rounded-full bg-emerald-400 shadow-[0_0_5px_rgba(52,211,153,0.8)]" />
                               Active
                             </span>
                           </div>
                        </div>
                      </div>
                      
                      <div className="flex flex-col gap-1.5">
                        <span className="text-[10px] font-mono uppercase tracking-widest text-slate-500 font-bold">Sandbox Preview</span>
                        <div className="p-4 rounded-lg border border-atc-border/50 bg-[#060a16] font-mono text-xs text-slate-400 overflow-x-auto">
                          {selectedFile.extension === 'ts' || selectedFile.extension === 'tsx' ? (
                            <pre className="text-atc-cyan/80">
                              <code>{`// ${selectedFile.name}\n// ${selectedFile.description}\n\nexport const init = async () => {\n  console.log("Initializing ${selectedFile.name}...");\n};`}</code>
                            </pre>
                          ) : selectedFile.extension === 'sol' ? (
                            <pre className="text-atc-purple/80">
                              <code>{`// SPDX-License-Identifier: MIT\npragma atc-lang ^0.8.0;\n\ncontract ${selectedFile.name.replace('.atc', '')} {\n    // Core logic\n}`}</code>
                            </pre>
                          ) : (
                            <pre className="text-slate-400">
                              <code>{`{\n  "file": "${selectedFile.name}",\n  "status": "ready"\n}`}</code>
                            </pre>
                          )}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center h-full text-center p-8 opacity-60">
              <div className="w-20 h-20 mb-6 rounded-full bg-[#090b14]/50 flex items-center justify-center border border-atc-border/50 shadow-[0_0_30px_rgba(0,0,0,0.5)]">
                <FileCode className="w-10 h-10 text-slate-500" />
              </div>
              <h3 className="text-lg font-medium text-white tracking-wide mb-2">Keine Datei ausgewählt</h3>
              <p className="text-sm text-slate-400 max-w-sm">
                Wähle eine Datei im Explorer auf der linken Seite aus, um deren Inhalt und Metadaten anzuzeigen.
              </p>
            </div>
          )}
        </div>
      </div>
      
      {contextMenu && (
        <div
          style={{
            top: contextMenu.y,
            left: contextMenu.x,
          }}
          className="fixed z-50 w-48 bg-[#0a0d16] border border-white/10 rounded-xl shadow-2xl overflow-hidden py-1"
          onClick={(e) => e.stopPropagation()}
        >
          <div className="px-3 py-2 border-b border-white/5 bg-white/[0.02]">
            <p className="text-xs font-mono text-slate-300 truncate">{contextMenu.node.name}</p>
          </div>
          <div className="p-1 font-mono text-xs text-slate-400">
            <button
              onClick={() => {
                setContextMenu(null);
                window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: `Renaming disabled for system modules.` }));
              }}
              className="w-full text-left px-3 py-1.5 hover:bg-white/10 hover:text-white rounded transition-colors"
            >
              Rename
            </button>
            <button
              onClick={() => {
                setContextMenu(null);
                window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: `System core integrity protection active. Deletion blocked.` }));
              }}
              className="w-full text-left px-3 py-1.5 hover:bg-white/10 hover:text-white rounded transition-colors"
            >
              Delete
            </button>
            <button
              onClick={() => {
                setContextMenu(null);
                setSelectedFile(contextMenu.node);
              }}
              className="w-full text-left px-3 py-1.5 hover:bg-white/10 hover:text-white rounded transition-colors"
            >
              Get Info
            </button>
            <button
              onClick={() => {
                setContextMenu(null);
                const path = `/${contextMenu.node.name}`;
                navigator.clipboard.writeText(path);
                window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: `Copied path: ${path}` }));
              }}
              className="w-full text-left px-3 py-1.5 hover:bg-white/10 hover:text-white rounded transition-colors"
            >
              Copy Path
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
