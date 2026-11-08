import React, { useState } from 'react';
import { Network, Server, Globe, Lock, Code, Database, Zap, Activity } from 'lucide-react';
import { motion } from 'motion/react';

export function ApiInterfacesView() {
  const [activeTab, setActiveTab] = useState<'rest'>('rest');

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200">
      <div className="p-6 border-b border-atc-border/50 bg-[#090b14]">
        <div className="flex items-center gap-3 mb-2">
          <div className="p-2 rounded-lg bg-emerald-500/10 border border-emerald-500/30">
            <Network className="w-6 h-6 text-emerald-400" />
          </div>
          <h1 className="text-2xl font-bold text-white tracking-tight">GlobusOS API Specification</h1>
        </div>
        <p className="text-slate-400 max-w-2xl">
          A-TownChain API Spezifikation (GlobusOS SDK) Version: 1.0.0-rc1.<br/>
          Basis-URL: <span className="font-mono text-emerald-400">http://localhost:8080/v1</span>
        </p>
      </div>

      <div className="flex flex-1 min-h-0">
        {/* Sidebar */}
        <div className="w-64 border-r border-atc-border/50 bg-[#090b14]/50 p-4 flex flex-col gap-2">
          <div className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-2 px-2">Schnittstellen (Gateways)</div>
          
          <button 
            onClick={() => setActiveTab('rest')}
            className={`flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-all ${activeTab === 'rest' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'}`}
          >
            <Globe className="w-4 h-4" /> REST API v1
          </button>

          <div className="mt-8 text-xs font-bold text-slate-500 uppercase tracking-widest mb-2 px-2">Access & Security</div>
          <button className="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-all text-slate-400 hover:bg-white/5 hover:text-slate-200">
            <Lock className="w-4 h-4" /> API Keys & OAuth
          </button>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
          {activeTab === 'rest' && (
            <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-8 max-w-4xl">
              <div>
                 <h2 className="text-xl font-bold text-white mb-2">GlobusOS REST Endpoints</h2>
                 <p className="text-slate-400">Dieses Dokument beschreibt die REST/JSON-Schnittstellen für die Interaktion mit der A-TownChain. Drittentwickler können diese Endpunkte nutzen, um dApps zu bauen.</p>
              </div>

              <div className="space-y-4">
                 {/* Endpoint 1 */}
                 <div className="border border-atc-border/50 rounded-xl bg-[#090b14] overflow-hidden">
                    <div className="flex items-center gap-3 px-4 py-3 border-b border-atc-border/50 bg-[#060a16]">
                       <span className="px-2 py-1 bg-emerald-500/20 text-emerald-400 text-xs font-bold rounded">GET</span>
                       <span className="font-mono text-sm text-slate-300">/chain/status</span>
                    </div>
                    <div className="p-4 flex flex-col gap-2 text-sm text-slate-400">
                       <p>Gibt den aktuellen Status der Blockchain und des Konsens-Moduls zurück.</p>
                       <div className="mt-2 text-xs font-mono bg-black/50 p-3 rounded-lg border border-white/5 whitespace-pre-wrap">
{`{
  "height": 15000000,
  "latest_hash": "0x123abc...",
  "active_validators": 142050,
  "network_tps": 12500,
  "poai_global_tflops": 8450000
}`}
                       </div>
                    </div>
                 </div>

                 {/* Endpoint 2 */}
                 <div className="border border-atc-border/50 rounded-xl bg-[#090b14] overflow-hidden">
                    <div className="flex items-center gap-3 px-4 py-3 border-b border-atc-border/50 bg-[#060a16]">
                       <span className="px-2 py-1 bg-emerald-500/20 text-emerald-400 text-xs font-bold rounded">GET</span>
                       <span className="font-mono text-sm text-slate-300">/account/{'{'}address{'}'}/balance</span>
                    </div>
                    <div className="p-4 flex flex-col gap-2 text-sm text-slate-400">
                       <p>Fragt das ATC-Guthaben einer quantensicheren Adresse ab.</p>
                       <div className="mt-2 text-xs font-mono bg-black/50 p-3 rounded-lg border border-white/5 whitespace-pre-wrap">
{`{
  "address": "ATC-Qabc123...",
  "balance_atc": 1450.25,
  "staked_atc": 10000.0,
  "poi_score": 85.5
}`}
                       </div>
                    </div>
                 </div>

                 {/* Endpoint 3 */}
                 <div className="border border-atc-border/50 rounded-xl bg-[#090b14] overflow-hidden">
                    <div className="flex items-center gap-3 px-4 py-3 border-b border-atc-border/50 bg-[#060a16]">
                       <span className="px-2 py-1 bg-blue-500/20 text-blue-400 text-xs font-bold rounded">POST</span>
                       <span className="font-mono text-sm text-slate-300">/vm/call</span>
                    </div>
                    <div className="p-4 flex flex-col gap-2 text-sm text-slate-400">
                       <p>Führt eine "Read-Only" Funktion eines Smart Contracts aus (kostet kein Gas).</p>
                       <div className="mt-2 text-xs font-mono bg-black/50 p-3 rounded-lg border border-white/5 whitespace-pre-wrap">
{`// Request
{
  "contract_address": "0xATC_Faucet...",
  "method": "get_greeting",
  "args": []
}`}
                       </div>
                    </div>
                 </div>

                 {/* Endpoint 4 */}
                 <div className="border border-atc-border/50 rounded-xl bg-[#090b14] overflow-hidden">
                    <div className="flex items-center gap-3 px-4 py-3 border-b border-atc-border/50 bg-[#060a16]">
                       <span className="px-2 py-1 bg-blue-500/20 text-blue-400 text-xs font-bold rounded">POST</span>
                       <span className="font-mono text-sm text-slate-300">/tx/broadcast</span>
                    </div>
                    <div className="p-4 flex flex-col gap-2 text-sm text-slate-400">
                       <p>Sendet eine signierte Transaktion an den Mempool. Dies ändert den State in der RocksDB und kostet ATC-Gas.</p>
                       <div className="mt-2 text-xs font-mono bg-black/50 p-3 rounded-lg border border-white/5 whitespace-pre-wrap">
{`// Request
{
  "contract_address": "0xTarget...",
  "caller": "ATC-Qabc123...",
  "method": "claim_tokens",
  "args": [],
  "state_hash_before": "0xabc...", 
  "signature_pq": "0xDilithiumSignatureData..."
}

// Response (Success)
{
  "status": "ACCEPTED",
  "tx_hash": "0x8f...4c1"
}

// Response (Blocked by ATC-Shield)
{
  "status": "SENTINEL_BLOCKED",
  "reason": "AI Sentinel Alert: Reentrancy pattern detected. Transaction frozen."
}`}
                       </div>
                    </div>
                 </div>

                 {/* Endpoint 5 */}
                 <div className="border border-atc-border/50 rounded-xl bg-[#090b14] overflow-hidden">
                    <div className="flex items-center gap-3 px-4 py-3 border-b border-atc-border/50 bg-[#060a16]">
                       <span className="px-2 py-1 bg-blue-500/20 text-blue-400 text-xs font-bold rounded">POST</span>
                       <span className="font-mono text-sm text-slate-300">/aifs/upload</span>
                    </div>
                    <div className="p-4 flex flex-col gap-2 text-sm text-slate-400">
                       <p>Verschlüsselt eine Datei und teilt sie in Shards für das P2P-Mesh auf.</p>
                       <div className="mt-2 text-xs font-mono bg-black/50 p-3 rounded-lg border border-white/5 whitespace-pre-wrap">
{`// Response
{
  "content_id": "AIFS-Hash-123",
  "shards_distributed": 32,
  "status": "SYNCED"
}`}
                       </div>
                    </div>
                 </div>

                 {/* Endpoint 6 */}
                 <div className="border border-atc-border/50 rounded-xl bg-[#090b14] overflow-hidden">
                    <div className="flex items-center gap-3 px-4 py-3 border-b border-atc-border/50 bg-[#060a16]">
                       <span className="px-2 py-1 bg-blue-500/20 text-blue-400 text-xs font-bold rounded">POST</span>
                       <span className="font-mono text-sm text-slate-300">/nexus/infer</span>
                    </div>
                    <div className="p-4 flex flex-col gap-2 text-sm text-slate-400">
                       <p>Fordert eine KI-Berechnung (z.B. LLM oder Data Processing) durch das globale PoAI-Miner-Netzwerk an.</p>
                       <div className="mt-2 text-xs font-mono bg-black/50 p-3 rounded-lg border border-white/5 whitespace-pre-wrap">
{`// Request
{
  "model_id": "GENESIS_LLM_V1",
  "prompt_tensor": [0.1, 0.5, 0.9, ...],
  "max_fee_atc": 0.05
}`}
                       </div>
                    </div>
                 </div>

              </div>
            </motion.div>
          )}

        </div>
      </div>
    </div>
  );
}
