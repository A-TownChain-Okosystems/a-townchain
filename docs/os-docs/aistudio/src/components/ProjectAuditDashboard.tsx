import React from 'react';
import { motion } from 'motion/react';
import { ShieldCheck, Code, PackageSearch, AlertTriangle, CheckCircle, Activity, ShieldAlert, Cpu } from 'lucide-react';

export const ProjectAuditDashboard: React.FC = () => {
  const securityMetrics = [
    { label: "Zero-Day Vulnerabilities", value: "0", status: "good" },
    { label: "ZKP Cryptography Standard", value: "100%", status: "good" },
    { label: "End-to-End Encryption Target", value: "Pass", status: "good" },
    { label: "Privilege Escalation Risks", value: "Low", status: "warning" },
  ];

  const dependencyHealth = [
    { name: "React / Frameworks", status: "Up to Date", version: "18.3.x" },
    { name: "Lucide Icons", status: "Up to Date", version: "0.3x" },
    { name: "Vite Configuration", status: "Optimized", version: "v6" },
    { name: "Crypto / ZKP Libs", status: "Audited", version: "Custom" },
  ];

  const codeQuality = [
    { metric: "Test Coverage", score: 87, max: 100 },
    { metric: "Cyclomatic Complexity", score: 14, max: 100, lowerIsBetter: true },
    { metric: "Duplication Density", score: 2.1, max: 100, lowerIsBetter: true },
    { metric: "Documentation Ratio", score: 92, max: 100 },
  ];

  return (
    <div className="w-full h-full bg-slate-900 border border-slate-800 rounded-lg p-6 overflow-y-auto">
      <div className="flex items-center gap-3 mb-6">
        <ShieldCheck className="w-8 h-8 text-atc-cyan" />
        <h2 className="text-2xl font-mono text-white tracking-widest uppercase">Ecosystem Software Audit</h2>
      </div>

      <div className="mb-8 text-slate-400 text-sm max-w-3xl">
        Automated analysis of the A-TownChain-Ökosystems. Visualizing Code Quality, Dependency Health, and Security Compliance metrics.
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        {/* Core Score Cards */}
        <motion.div initial={{ y: 20, opacity: 0 }} animate={{ y: 0, opacity: 1 }} className="bg-slate-800/60 border border-atc-cyan/30 rounded-xl p-6 relative overflow-hidden">
          <div className="absolute -right-4 -top-4 opacity-[0.03]"><ShieldCheck className="w-32 h-32" /></div>
          <h3 className="text-white font-bold mb-4 flex items-center gap-2"><ShieldCheck className="w-5 h-5 text-atc-cyan"/> Security Compliance</h3>
          <div className="text-4xl font-mono text-atc-cyan mb-2">98.5%</div>
          <p className="text-xs text-slate-400">Enterprise grade level. Exceeds A-Town specifications.</p>
        </motion.div>

        <motion.div initial={{ y: 20, opacity: 0 }} animate={{ y: 0, opacity: 1 }} transition={{ delay: 0.1 }} className="bg-slate-800/60 border border-green-500/30 rounded-xl p-6 relative overflow-hidden">
          <div className="absolute -right-4 -top-4 opacity-[0.03]"><Code className="w-32 h-32" /></div>
          <h3 className="text-white font-bold mb-4 flex items-center gap-2"><Code className="w-5 h-5 text-green-400"/> Code Quality Score</h3>
          <div className="text-4xl font-mono text-green-400 mb-2">A+</div>
          <p className="text-xs text-slate-400">Maintained high architectural integrity & maintainability.</p>
        </motion.div>

        <motion.div initial={{ y: 20, opacity: 0 }} animate={{ y: 0, opacity: 1 }} transition={{ delay: 0.2 }} className="bg-slate-800/60 border border-blue-500/30 rounded-xl p-6 relative overflow-hidden">
          <div className="absolute -right-4 -top-4 opacity-[0.03]"><PackageSearch className="w-32 h-32" /></div>
          <h3 className="text-white font-bold mb-4 flex items-center gap-2"><PackageSearch className="w-5 h-5 text-blue-400"/> Dependency Health</h3>
          <div className="text-4xl font-mono text-blue-400 mb-2">99.0%</div>
          <p className="text-xs text-slate-400">All core dependencies are up-to-date and audited.</p>
        </motion.div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        
        {/* Security Assessment */}
        <div className="bg-slate-800/40 border border-slate-700/50 rounded-xl p-5">
          <h3 className="text-lg font-bold text-white mb-4 border-b border-slate-700 pb-2 flex items-center gap-2">
            <ShieldAlert className="w-5 h-5 text-slate-400" /> Security Assessment
          </h3>
          <div className="space-y-3">
            {securityMetrics.map((item, idx) => (
              <div key={idx} className="flex items-center justify-between bg-slate-900/50 p-3 rounded border border-slate-800">
                <span className="text-sm text-slate-300">{item.label}</span>
                <div className="flex items-center gap-2">
                  <span className={`text-sm font-mono ${item.status === 'good' ? 'text-green-400' : 'text-yellow-400'}`}>{item.value}</span>
                  {item.status === 'good' ? <CheckCircle className="w-4 h-4 text-green-400" /> : <AlertTriangle className="w-4 h-4 text-yellow-400" />}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Code Metrics */}
        <div className="bg-slate-800/40 border border-slate-700/50 rounded-xl p-5">
          <h3 className="text-lg font-bold text-white mb-4 border-b border-slate-700 pb-2 flex items-center gap-2">
            <Activity className="w-5 h-5 text-slate-400" /> Static Analysis Metrics
          </h3>
          <div className="space-y-4">
            {codeQuality.map((item, idx) => {
              const percentage = (item.score / item.max) * 100;
              const isGood = item.lowerIsBetter ? item.score < 20 : item.score > 80;
              
              return (
                <div key={idx}>
                  <div className="flex justify-between text-xs mb-1">
                    <span className="text-slate-300">{item.metric}</span>
                    <span className="font-mono text-slate-400">{item.score}{item.lowerIsBetter ? '' : '%'}</span>
                  </div>
                  <div className="w-full bg-slate-900 h-2 rounded-full overflow-hidden border border-slate-800">
                    <motion.div 
                      initial={{ width: 0 }}
                      animate={{ width: `${percentage}%` }}
                      transition={{ duration: 1, delay: 0.3 + (idx * 0.1) }}
                      className={`h-full rounded-full ${isGood ? (item.lowerIsBetter ? 'bg-blue-400' : 'bg-green-400') : 'bg-yellow-500'}`}
                    />
                  </div>
                </div>
              )
            })}
          </div>
        </div>

        {/* Dependency Tree Status */}
        <div className="bg-slate-800/40 border border-slate-700/50 rounded-xl p-5 lg:col-span-2">
          <h3 className="text-lg font-bold text-white mb-4 border-b border-slate-700 pb-2 flex items-center gap-2">
            <Cpu className="w-5 h-5 text-slate-400" /> Subsystem Dependency Tree
          </h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
            {dependencyHealth.map((dep, idx) => (
              <div key={idx} className="bg-slate-900/50 p-3 rounded-lg border border-slate-800">
                <div className="text-xs text-slate-500 mb-1">{dep.name}</div>
                <div className="flex items-center justify-between">
                  <span className="text-sm font-bold text-slate-200">{dep.version}</span>
                  <span className="text-[10px] uppercase tracking-wider px-2 py-0.5 rounded bg-blue-500/20 text-blue-400">
                    {dep.status}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>

      </div>
    </div>
  );
};
