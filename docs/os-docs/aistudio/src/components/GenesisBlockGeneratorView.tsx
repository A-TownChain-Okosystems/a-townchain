import React, { useState } from 'react';
import { Database, FileJson, Save, Cpu, HardDrive, Key, Play } from 'lucide-react';

export function GenesisBlockGeneratorView() {
  const [networkName, setNetworkName] = useState("A-TownChain Mainnet");
  const [chainId, setChainId] = useState("atc-1");
  const [initialSupply, setInitialSupply] = useState(1000000000);
  const [founderWallet, setFounderWallet] = useState("atc1qxxz...");
  const [generatedGenesis, setGeneratedGenesis] = useState<string | null>(null);

  const generateGenesis = () => {
    const genesisData = {
        "genesis_time": new Date().toISOString(),
        "chain_id": chainId,
        "initial_height": "1",
        "consensus_params": {
            "block": {
                "max_bytes": "22020096",
                "max_gas": "-1",
                "time_iota_ms": "1000"
            },
            "evidence": {
                "max_age_num_blocks": "100000",
                "max_age_duration": "172800000000000",
                "max_bytes": "1048576"
            },
            "validator": {
                "pub_key_types": [
                    "ed25519"
                ]
            },
            "poai": {
                "loss_threshold": 0.2,
                "reward_multiplier": 1.5
            }
        },
        "app_hash": "",
        "app_state": {
            "bank": {
                "balances": [
                    {
                        "address": founderWallet || "atc1founder",
                        "coins": [
                            {
                                "denom": "uatc",
                                "amount": (initialSupply * 1000000).toString()
                            }
                        ]
                    }
                ],
                "supply": [
                    {
                        "denom": "uatc",
                        "amount": (initialSupply * 1000000).toString()
                    }
                ]
            },
            "governance": {
                "voting_period": "172800s",
                "minimum_deposit": [
                    {
                        "denom": "uatc",
                        "amount": "10000000"
                    }
                ]
            }
        }
    };
    
    setGeneratedGenesis(JSON.stringify(genesisData, null, 4));
  };

  return (
    <div className="flex flex-col h-full bg-[#050B14] border border-white/5 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center gap-3 px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <Database className="w-5 h-5 text-emerald-400" />
        <div>
          <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">Genesis-Block-Generator</h2>
          <p className="text-xs text-slate-400 mt-0.5">Initialisierung des A-TownChain Netzwerks</p>
        </div>
      </div>
      
      <div className="flex-1 flex overflow-hidden">
        {/* Left Side: Form */}
        <div className="w-1/2 p-6 overflow-y-auto custom-scrollbar border-r border-white/5">
            <h3 className="font-mono text-sm text-slate-300 uppercase tracking-widest mb-6 flex items-center gap-2">
                <Cpu className="w-4 h-4 text-slate-400" /> Parameter
            </h3>

            <div className="space-y-4">
                <div>
                   <label className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-1.5 block">Network Name</label>
                   <input type="text" value={networkName} onChange={e => setNetworkName(e.target.value)} className="w-full bg-[#020408] border border-white/10 rounded-lg px-3 py-2 text-xs font-mono text-slate-300 outline-none focus:border-emerald-500/50" />
                </div>
                <div>
                   <label className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-1.5 block">Chain ID</label>
                   <input type="text" value={chainId} onChange={e => setChainId(e.target.value)} className="w-full bg-[#020408] border border-white/10 rounded-lg px-3 py-2 text-xs font-mono text-slate-300 outline-none focus:border-emerald-500/50" />
                </div>
                <div>
                   <label className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-1.5 block">Initial Supply (ATC)</label>
                   <input type="number" value={initialSupply} onChange={e => setInitialSupply(parseInt(e.target.value) || 0)} className="w-full bg-[#020408] border border-white/10 rounded-lg px-3 py-2 text-xs font-mono text-slate-300 outline-none focus:border-emerald-500/50" />
                </div>
                <div>
                   <label className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-1.5 block">Founder Wallet (Bech32)</label>
                   <input type="text" value={founderWallet} onChange={e => setFounderWallet(e.target.value)} className="w-full bg-[#020408] border border-white/10 rounded-lg px-3 py-2 text-xs font-mono text-slate-300 outline-none focus:border-emerald-500/50" />
                </div>
            </div>

            <div className="mt-8">
               <button 
                  onClick={generateGenesis}
                  className="w-full px-4 py-3 bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 hover:bg-emerald-500/20 rounded-lg font-mono text-xs uppercase tracking-widest transition-colors flex items-center justify-center gap-2"
               >
                  <Play className="w-4 h-4" /> Build Genesis JSON
               </button>
            </div>
        </div>

        {/* Right Side: Output */}
        <div className="w-1/2 bg-[#020408] flex flex-col relative">
           <div className="absolute top-4 right-4 flex gap-2">
              <button 
                disabled={!generatedGenesis}
                className="p-1.5 bg-slate-800 text-slate-300 hover:text-white hover:bg-slate-700 rounded border border-white/10 disabled:opacity-50 disabled:cursor-not-allowed"
                title="Save genesis.json"
              >
                  <Save className="w-4 h-4" />
              </button>
           </div>
           <div className="p-4 border-b border-white/5 flex items-center gap-2">
              <FileJson className="w-4 h-4 text-emerald-400" />
              <span className="font-mono text-xs text-slate-400">genesis.json</span>
           </div>
           <div className="flex-1 p-4 overflow-y-auto custom-scrollbar">
               {generatedGenesis ? (
                   <pre className="font-mono text-[10px] text-emerald-400/80 whitespace-pre-wrap leading-relaxed">
                       {generatedGenesis}
                   </pre>
               ) : (
                   <div className="h-full flex flex-col items-center justify-center text-slate-500 opacity-50">
                       <HardDrive className="w-12 h-12 mb-4" />
                       <span className="font-mono text-xs uppercase tracking-widest">Awaiting generation</span>
                   </div>
               )}
           </div>
        </div>
      </div>
    </div>
  );
}
