import React from 'react';
import { Webhook, Radio, Zap, Server, Activity } from 'lucide-react';

export function InterfacesView() {
  const interfaces = [
    { name: "Public REST API", type: "Gateway", protocol: "HTTPS", status: "Operational", rps: "12,400" },
    { name: "Real-time Blockchain Event Stream", type: "WebSocket", protocol: "WSS", status: "Operational", rps: "54,200 msg/s" },
    { name: "Node Inter-RPC", type: "Internal", protocol: "gRPC", status: "Operational", rps: "8,900" },
    { name: "Legacy Sync Hook", type: "WebHook", protocol: "HTTP", status: "Deprecated", rps: "12" }
  ];

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-pink-500/10 border border-pink-500/30 flex items-center justify-center text-pink-400 shadow-[0_0_15px_rgba(236,72,153,0.2)]">
          <Webhook className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Schnittstellen & Gateways
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Übersicht aller APIs, WebSockets und System-Schnittstellen für externe und interne Kommunikation.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {interfaces.map((api, i) => (
          <div key={i} className="p-6 rounded-2xl bg-[#090b14] border border-white/5 hover:border-pink-500/30 transition-colors flex flex-col gap-4">
            <div className="flex justify-between items-start">
              <div className="flex items-center gap-3">
                <div className={`p-2 rounded-lg ${api.protocol === 'WSS' ? 'bg-blue-500/10 text-blue-400' : api.protocol === 'gRPC' ? 'bg-purple-500/10 text-purple-400' : 'bg-pink-500/10 text-pink-400'}`}>
                  {api.protocol === 'WSS' ? <Radio className="w-5 h-5" /> : api.protocol === 'gRPC' ? <Server className="w-5 h-5" /> : <Zap className="w-5 h-5" />}
                </div>
                <div>
                  <h3 className="font-semibold text-slate-200">{api.name}</h3>
                  <div className="text-xs font-mono text-slate-500 mt-1">{api.protocol} • {api.type}</div>
                </div>
              </div>
              <span className={`px-2 py-1 text-[10px] font-bold uppercase rounded ${
                api.status === 'Operational' ? 'bg-emerald-500/10 text-emerald-400' : 'bg-red-500/10 text-red-400'
              }`}>
                {api.status}
              </span>
            </div>
            <div className="pt-4 border-t border-white/5 flex items-center justify-between text-sm">
              <span className="text-slate-400 flex items-center gap-2"><Activity className="w-4 h-4 text-slate-500" /> Throughput</span>
              <span className="font-mono text-white">{api.rps}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
