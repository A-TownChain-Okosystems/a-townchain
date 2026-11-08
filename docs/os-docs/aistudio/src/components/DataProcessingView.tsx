import React from 'react';
import { DatabaseZap, Filter, BarChart4, CloudLightning, Shield } from 'lucide-react';

export function DataProcessingView() {
  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-[0_0_15px_rgba(6,182,212,0.2)]">
          <DatabaseZap className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Data Processing & ETL
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Überwachung der Datenverarbeitungspipelines, Analytics Ingestion und DSGVO-Compliance-Filter.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5">
          <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2">
            <CloudLightning className="w-5 h-5 text-cyan-400" /> Pipeline Streams
          </h3>
          <div className="space-y-4">
            <div className="p-4 rounded-xl bg-black/40 border border-white/5 relative overflow-hidden">
               <div className="absolute top-0 left-0 h-full w-1 bg-cyan-500"></div>
               <div className="flex justify-between items-start mb-2">
                 <span className="font-medium text-slate-200">User Telemetry Ingestion</span>
                 <span className="text-xs font-mono text-cyan-400">4.2 GB/s</span>
               </div>
               <p className="text-xs text-slate-500 mb-3">Kafka Topic: atc.telemetry.events</p>
               <div className="w-full bg-white/5 rounded-full h-1.5">
                 <div className="bg-cyan-500 h-1.5 rounded-full" style={{ width: '65%' }}></div>
               </div>
            </div>
            
            <div className="p-4 rounded-xl bg-black/40 border border-white/5 relative overflow-hidden">
               <div className="absolute top-0 left-0 h-full w-1 bg-purple-500"></div>
               <div className="flex justify-between items-start mb-2">
                 <span className="font-medium text-slate-200">On-Chain State Indexer</span>
                 <span className="text-xs font-mono text-purple-400">12,000 blocks/hr</span>
               </div>
               <p className="text-xs text-slate-500 mb-3">PostgreSQL Sync process</p>
               <div className="w-full bg-white/5 rounded-full h-1.5">
                 <div className="bg-purple-500 h-1.5 rounded-full" style={{ width: '92%' }}></div>
               </div>
            </div>
          </div>
        </div>

        <div className="flex flex-col gap-6">
          <div className="p-6 rounded-2xl bg-gradient-to-br from-indigo-900/20 to-black border border-indigo-500/20">
             <h3 className="text-sm font-semibold text-indigo-400 mb-4 flex items-center gap-2">
               <Filter className="w-4 h-4" /> ETL Transformation Rules
             </h3>
             <ul className="text-xs text-slate-400 space-y-2 font-mono">
               <li>1. PII Anonymization (SHA-256)</li>
               <li>2. IP Geolocation Enrichment</li>
               <li>3. JSON Flattening</li>
               <li>4. Parquet Conversion</li>
             </ul>
          </div>

          <div className="p-6 rounded-2xl bg-emerald-500/5 border border-emerald-500/20">
             <h3 className="text-sm font-semibold text-emerald-500 mb-2 flex items-center gap-2">
               <Shield className="w-4 h-4" /> DSGVO / GDPR Enclave
             </h3>
             <p className="text-xs text-emerald-200/70 leading-relaxed">
               All data processing streams run through the privacy enclave. Right to be forgotten rules are applied automatically via daily cron jobs. No raw PII is written to long-term cold storage.
             </p>
          </div>
        </div>
      </div>
    </div>
  );
}
