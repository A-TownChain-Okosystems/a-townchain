import React from 'react';
import { ShieldCheck, Eye, EyeOff, Lock, UserCheck, Key, FileText, Database, Server, Fingerprint } from 'lucide-react';
import { motion } from 'motion/react';
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid, AreaChart, Area } from 'recharts';

export function ComplianceView() {
  const complianceFeatures = [
    {
      id: 'zk',
      title: 'Zero-Knowledge Proofs (ZK-SNARKs)',
      description: 'ZKP-Engine validates transactions without exposing sensitive payload data.',
      icon: EyeOff,
      status: 'Active',
      level: 'High'
    },
    {
      id: 'vc',
      title: 'Verifiable Credentials (VCs)',
      description: 'Selective disclosure of DID attributes complying with GDPR data minimization.',
      icon: UserCheck,
      status: 'Active',
      level: 'High'
    },
    {
      id: 'offchain',
      title: 'Off-Chain Enclaves',
      description: 'PII stored in encrypted enclaves. Only hashes exist on-chain (Right to be Forgotten).',
      icon: Database,
      status: 'Active',
      level: 'Critical'
    },
    {
      id: 'viewkey',
      title: 'Audit View Keys',
      description: 'Opt-in transparency keys for regulatory auditors and tax compliance (MiCA).',
      icon: Key,
      status: 'Deployed',
      level: 'High'
    },
    {
      id: 'stealth',
      title: 'Stealth Addresses',
      description: 'One-time deterministically generated addresses for transaction untraceability.',
      icon: Fingerprint,
      status: 'Active',
      level: 'Medium'
    },
    {
      id: 'aml',
      title: 'AML-Gates & Oracles',
      description: 'DeFi protocols gated by ZK-based Anti-Money Laundering checks.',
      icon: ShieldCheck,
      status: 'Active',
      level: 'Critical'
    }
  ];

  const auditData = [
    { name: 'Mon', requests: 12, validations: 12 },
    { name: 'Tue', requests: 19, validations: 19 },
    { name: 'Wed', requests: 15, validations: 14 },
    { name: 'Thu', requests: 22, validations: 22 },
    { name: 'Fri', requests: 30, validations: 29 },
    { name: 'Sat', requests: 10, validations: 10 },
    { name: 'Sun', requests: 8, validations: 8 }
  ];

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-300 rounded-xl border border-white/10 overflow-hidden p-6 gap-6 relative">
      <div className="absolute top-0 left-0 w-full h-[2px] bg-gradient-to-r from-transparent via-emerald-500/50 to-transparent"></div>
      
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="p-3 bg-emerald-500/10 rounded-xl border border-emerald-500/20">
            <ShieldCheck className="w-6 h-6 text-emerald-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-white uppercase tracking-widest font-mono">Privacy & Compliance Engine</h2>
            <p className="text-sm text-slate-400 mt-1">GDPR & MiCA regulatory frameworks mapping to A-TownChain architecture</p>
          </div>
        </div>
        <div className="flex items-center gap-4">
           <div className="px-4 py-2 bg-emerald-500/10 border border-emerald-500/30 rounded-lg flex items-center gap-2">
             <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
             <span className="text-xs font-mono text-emerald-400 font-bold tracking-widest">DSGVO COMPLIANT</span>
           </div>
           <div className="px-4 py-2 bg-blue-500/10 border border-blue-500/30 rounded-lg flex items-center gap-2">
             <span className="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></span>
             <span className="text-xs font-mono text-blue-400 font-bold tracking-widest">MiCA READY</span>
           </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {complianceFeatures.map((feat) => (
          <motion.div 
            key={feat.id} 
            whileHover={{ y: -2 }}
            className="p-5 rounded-xl border border-white/10 bg-[#090b14] hover:bg-white/5 transition-colors relative overflow-hidden group"
          >
            <div className="flex items-start justify-between mb-4">
              <div className="p-2 bg-white/5 rounded-lg group-hover:bg-emerald-500/10 transition-colors">
                <feat.icon className="w-5 h-5 text-slate-400 group-hover:text-emerald-400 transition-colors" />
              </div>
              <span className="text-[10px] font-mono tracking-widest px-2 py-1 bg-white/5 rounded text-slate-400 uppercase">
                {feat.status}
              </span>
            </div>
            <h3 className="text-base font-bold text-slate-200 mb-2">{feat.title}</h3>
            <p className="text-xs text-slate-400 leading-relaxed mb-4">{feat.description}</p>
            <div className={`text-[10px] uppercase font-bold tracking-wider float-right px-2 py-1 rounded bg-black/40 ${feat.level === 'Critical' ? 'text-rose-400' : 'text-amber-400'}`}>
              Priority: {feat.level}
            </div>
          </motion.div>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-2 flex-1">
        
        {/* Compliance Telemetry Chart */}
        <div className="bg-[#090b14] border border-white/10 p-5 rounded-xl flex flex-col">
          <div className="flex items-center gap-2 mb-6">
             <FileText className="w-4 h-4 text-emerald-400" />
             <h3 className="text-sm font-mono tracking-widest text-slate-300 font-bold">ZK-Audits & VC Requests (7 Days)</h3>
          </div>
          <div className="flex-1 min-h-[200px]">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={auditData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
                <defs>
                  <linearGradient id="colorValidations" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#10b981" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#10b981" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} />
                <XAxis dataKey="name" stroke="#64748b" tick={{fill: '#64748b', fontSize: 10}} axisLine={false} tickLine={false} />
                <YAxis stroke="#64748b" tick={{fill: '#64748b', fontSize: 10}} axisLine={false} tickLine={false} />
                <Tooltip 
                  contentStyle={{ backgroundColor: '#060a16', border: '1px solid #1e293b', borderRadius: '8px' }}
                  itemStyle={{ fontSize: '12px' }}
                  labelStyle={{ color: '#94a3b8', fontSize: '12px' }}
                />
                <Area type="monotone" dataKey="requests" stroke="#6366f1" fillOpacity={0} strokeWidth={2} name="VC Requests" />
                <Area type="monotone" dataKey="validations" stroke="#10b981" fill="url(#colorValidations)" strokeWidth={2} name="ZK Validations" />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Console / Status output */}
        <div className="bg-black/60 border border-white/10 p-5 rounded-xl font-mono text-xs text-slate-300 flex flex-col">
           <div className="flex items-center justify-between mb-4 border-b border-white/10 pb-4">
             <div className="flex items-center gap-2 text-slate-400">
               <Server className="w-4 h-4 text-blue-400" />
               <span className="uppercase tracking-widest">Compliance Node Status</span>
             </div>
             <span className="text-emerald-400 animate-pulse">● LIVE</span>
           </div>
           <div className="space-y-3 flex-1 overflow-y-auto">
             <div className="flex gap-4">
               <span className="text-slate-600">14:02:11</span>
               <span className="text-indigo-400">[ZK-ENGINE]</span>
               <span>Proof verified for tx_hash: 0x8f...39a</span>
             </div>
             <div className="flex gap-4">
               <span className="text-slate-600">14:02:29</span>
               <span className="text-blue-400">[VC-GATE]</span>
               <span>KYC attribute selectively disclosed (Over 18)</span>
             </div>
             <div className="flex gap-4">
               <span className="text-slate-600">14:02:45</span>
               <span className="text-emerald-400">[DID-REGISTRY]</span>
               <span>New pairwise DID registered successfully</span>
             </div>
             <div className="flex gap-4">
               <span className="text-slate-600">14:03:10</span>
               <span className="text-rose-400">[AML-ORACLE]</span>
               <span>Blocked interaction from sanctioned address cluster</span>
             </div>
             <div className="flex gap-4">
               <span className="text-slate-600">14:03:52</span>
               <span className="text-amber-400">[ENCLAVE]</span>
               <span>Data chunk securely hashed. PII moved to off-chain vault.</span>
             </div>
           </div>
        </div>

      </div>
    </div>
  );
}
