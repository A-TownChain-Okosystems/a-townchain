import React, { useState } from 'react';
import { Coins, ArrowDownUp, Settings, Activity, Plus, FileCode, Shield } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function DeFiLiquidityPoolView() {
  const [activeTab, setActiveTab] = useState<'swap' | 'pool' | 'contract'>('swap');
  const [fromAmount, setFromAmount] = useState('100');
  const [toAmount, setToAmount] = useState('245.50');
  const [isSwapping, setIsSwapping] = useState(false);

  const handleSwap = () => {
    setIsSwapping(true);
    setTimeout(() => {
      setIsSwapping(false);
      setFromAmount('');
      setToAmount('');
    }, 1500);
  };

  return (
    <div className="flex flex-col h-full bg-[#050B14] border border-white/5 rounded-xl overflow-hidden font-sans">
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-indigo-500/10 flex items-center justify-center border border-indigo-500/30">
             <Coins className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">A-Town Liquidity Layer</h2>
            <p className="text-xs text-slate-400 mt-0.5">Automated Market Maker & DeFi Smart Contract</p>
          </div>
        </div>
        
        <div className="flex bg-[#020408] rounded-lg p-1 border border-white/5">
           <TabButton active={activeTab === 'swap'} onClick={() => setActiveTab('swap')} icon={<ArrowDownUp />} label="Swap" />
           <TabButton active={activeTab === 'pool'} onClick={() => setActiveTab('pool')} icon={<Plus />} label="Liquidity" />
           <TabButton active={activeTab === 'contract'} onClick={() => setActiveTab('contract')} icon={<FileCode />} label="Smart Contract" />
        </div>
      </div>
      
      {/* Content Area */}
      <div className="flex-1 overflow-y-auto custom-scrollbar relative bg-[#020408]">
          {/* Neon Background Accents */}
          <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-indigo-500/5 rounded-full blur-[100px] pointer-events-none" />
          <div className="absolute bottom-1/4 right-1/4 w-96 h-96 bg-purple-500/5 rounded-full blur-[100px] pointer-events-none" />

          <div className="p-8 flex justify-center min-h-full">
             <AnimatePresence mode="wait">
                 {activeTab === 'swap' && (
                    <motion.div 
                       key="swap"
                       initial={{ opacity: 0, y: 10 }}
                       animate={{ opacity: 1, y: 0 }}
                       exit={{ opacity: 0, y: -10 }}
                       className="w-full max-w-md"
                    >
                        <div className="bg-[#050B14] border border-white/10 rounded-2xl p-4 shadow-xl z-10 relative">
                            <div className="flex justify-between items-center mb-4">
                               <h3 className="font-mono text-sm text-slate-300 font-bold uppercase tracking-wider">Swap</h3>
                               <button className="text-slate-500 hover:text-white transition-colors">
                                   <Settings className="w-4 h-4" />
                               </button>
                            </div>

                            {/* From Box */}
                            <div className="bg-[#020408] p-4 rounded-xl border border-white/5 relative">
                                <label className="text-[10px] uppercase font-mono text-slate-500 tracking-widest block mb-2">You Pay</label>
                                <div className="flex justify-between items-center">
                                    <input 
                                       type="number" 
                                       value={fromAmount}
                                       onChange={(e) => setFromAmount(e.target.value)}
                                       className="bg-transparent text-3xl text-white font-mono outline-none w-1/2" 
                                       placeholder="0.0"
                                    />
                                    <div className="flex items-center gap-2 bg-indigo-500/20 px-3 py-1.5 rounded-lg border border-indigo-500/30">
                                        <div className="w-4 h-4 rounded-full bg-indigo-400" />
                                        <span className="font-mono text-sm font-bold text-indigo-300">ATC</span>
                                    </div>
                                </div>
                                <div className="text-[10px] text-slate-500 font-mono mt-2 text-right">Balance: 1450.25 ATC</div>
                            </div>

                            {/* Swap Icon */}
                            <div className="flex justify-center -my-3 relative z-10">
                                <button className="bg-slate-800 p-2 rounded-xl border border-white/10 text-slate-400 hover:text-white hover:border-indigo-500/50 transition-all">
                                   <ArrowDownUp className="w-4 h-4" />
                                </button>
                            </div>

                            {/* To Box */}
                            <div className="bg-[#020408] p-4 rounded-xl border border-white/5">
                                <label className="text-[10px] uppercase font-mono text-slate-500 tracking-widest block mb-2">You Receive</label>
                                <div className="flex justify-between items-center">
                                    <input 
                                       type="number" 
                                       value={toAmount}
                                       onChange={(e) => setToAmount(e.target.value)}
                                       className="bg-transparent text-3xl text-white font-mono outline-none w-1/2" 
                                       placeholder="0.0"
                                    />
                                    <div className="flex items-center gap-2 bg-green-500/10 px-3 py-1.5 rounded-lg border border-green-500/20">
                                        <span className="font-mono text-sm font-bold text-green-400">aUSD</span>
                                    </div>
                                </div>
                                <div className="text-[10px] text-slate-500 font-mono mt-2 flex justify-between">
                                    <span>1 ATC = 2.455 aUSD</span>
                                    <span>Balance: 0.00 aUSD</span>
                                </div>
                            </div>

                            <button 
                               onClick={handleSwap}
                               disabled={isSwapping}
                               className={`w-full mt-4 py-4 rounded-xl font-mono uppercase tracking-widest font-bold text-sm transition-all relative overflow-hidden ${
                                   isSwapping 
                                   ? 'bg-indigo-500/50 text-white/50 cursor-not-allowed' 
                                   : 'bg-indigo-500 text-white hover:bg-indigo-400 shadow-[0_0_20px_rgba(99,102,241,0.3)]'
                               }`}
                            >
                               {isSwapping ? (
                                   <span className="flex items-center justify-center gap-2">
                                       <Activity className="w-4 h-4 animate-spin" /> Processing via ZK-Rollup...
                                   </span>
                               ) : (
                                   "Swap Assets"
                               )}
                            </button>
                        </div>
                        
                        <div className="mt-6 flex justify-between px-4 text-[10px] font-mono text-slate-500">
                            <span className="flex items-center gap-1"><Shield className="w-3 h-3" /> ATC-Shield Protected</span>
                            <span>Gas Fee: 0.0001 ATC</span>
                        </div>
                    </motion.div>
                 )}

                 {activeTab === 'contract' && (
                     <motion.div 
                        key="contract"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        className="w-full max-w-3xl"
                     >
                         <div className="bg-[#050B14] border border-white/5 rounded-xl overflow-hidden">
                             <div className="px-4 py-3 border-b border-white/5 bg-black/40 flex items-center justify-between">
                                 <span className="font-mono text-xs text-indigo-400 uppercase tracking-widest">WASM Smart Contract (Rust)</span>
                                 <span className="text-[10px] bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded uppercase">Verified</span>
                             </div>
                             <div className="p-4 font-mono text-xs leading-relaxed text-slate-300 overflow-x-auto">
<pre>{`use atc_std::{msg, env, storage, math::U256};
use atvm_macros::contract;

#[contract]
pub struct AtcLiquidityPool {
    reserve_atc: U256,
    reserve_token: U256,
}

#[contract_impl]
impl AtcLiquidityPool {
    /// Initialisiert den Liquidity Pool mit dem Constant Product Market Maker (CPMM) Modell.
    /// Formel: x * y = k
    pub fn new(initial_atc: U256, initial_token: U256) -> Self {
        Self {
            reserve_atc: initial_atc,
            reserve_token: initial_token,
        }
    }

    /// Tauscht ATC gegen den Ziel-Token.
    pub fn swap_atc_to_token(&mut self, atc_in: U256) -> U256 {
        let fee_adjusted_in = atc_in * U256::from(997) / U256::from(1000); // 0.3% Fee
        
        let numerator = self.reserve_token * fee_adjusted_in;
        let denominator = self.reserve_atc + fee_adjusted_in;
        let token_out = numerator / denominator;
        
        self.reserve_atc += atc_in;
        self.reserve_token -= token_out;
        
        token_out
    }
}`}</pre>
                             </div>
                         </div>
                     </motion.div>
                 )}
                 
                 {activeTab === 'pool' && (
                      <motion.div 
                        key="pool"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        className="w-full max-w-2xl"
                      >
                           <div className="grid grid-cols-2 gap-6">
                               <div className="bg-[#050B14] p-6 border border-white/10 rounded-2xl">
                                  <h3 className="font-mono text-sm text-slate-300 font-bold uppercase tracking-wider mb-6">Pool Statistics</h3>
                                  
                                  <div className="space-y-4">
                                      <div>
                                          <div className="text-[10px] text-slate-500 font-mono uppercase tracking-widest mb-1">Total Liquidity</div>
                                          <div className="text-2xl font-mono text-white">$14.5M</div>
                                      </div>
                                      <div>
                                          <div className="text-[10px] text-slate-500 font-mono uppercase tracking-widest mb-1">24h Volume</div>
                                          <div className="text-xl font-mono text-indigo-400">$2.1M</div>
                                      </div>
                                      <div>
                                          <div className="text-[10px] text-slate-500 font-mono uppercase tracking-widest mb-1">Fee Tier</div>
                                          <div className="text-sm font-mono text-slate-300">0.3%</div>
                                      </div>
                                  </div>
                               </div>

                               <div className="bg-indigo-500/10 p-6 border border-indigo-500/20 rounded-2xl flex flex-col items-center justify-center text-center">
                                   <div className="w-16 h-16 rounded-full bg-indigo-500/20 flex items-center justify-center mb-4 text-indigo-400 border border-indigo-400/30">
                                       <Plus className="w-8 h-8" />
                                   </div>
                                   <h3 className="font-mono text-sm text-white font-bold uppercase tracking-wider mb-2">Add Liquidity</h3>
                                   <p className="text-xs text-indigo-200/70 mb-6">Earn fees by providing liquidity to the ATC/aUSD pool.</p>
                                   <button className="px-6 py-2 bg-indigo-500 hover:bg-indigo-400 text-white rounded font-mono text-xs uppercase tracking-widest font-bold transition-colors">
                                       Deposit
                                   </button>
                               </div>
                           </div>
                      </motion.div>
                 )}
             </AnimatePresence>
          </div>
      </div>
    </div>
  );
}

function TabButton({ active, onClick, icon, label }: { active: boolean, onClick: () => void, icon: React.ReactNode, label: string }) {
    return (
        <button 
           onClick={onClick}
           className={`flex items-center gap-2 px-4 py-1.5 rounded-md text-xs font-mono uppercase tracking-wider transition-all ${
               active 
               ? 'bg-slate-800 text-white shadow-sm' 
               : 'text-slate-500 hover:text-slate-300 hover:bg-slate-800/50'
           }`}
        >
            <div className="w-3.5 h-3.5 flex items-center justify-center">
               {icon}
            </div>
            {label}
        </button>
    );
}
