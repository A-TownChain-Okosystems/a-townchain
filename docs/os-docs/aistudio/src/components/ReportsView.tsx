import React, { useRef, useState, useMemo } from 'react';
import { Download, FileText, Activity, Layers, AlertCircle, RefreshCw } from 'lucide-react';
import jsPDF from 'jspdf';
import html2canvas from 'html2canvas';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

import { TARGET_STATE, LAYERS } from '../data';
import { TIERS } from '../tierData';
import { ARCHITECTURE_REVIEW, REPOSITORY_REVIEW, ENTERPRISE_GAP_ANALYSIS } from '../auditData';

export function ReportsView() {
  const [isGenerating, setIsGenerating] = useState(false);
  const reportRef = useRef<HTMLDivElement>(null);

  const syncData = useMemo(() => {
    const data = [];
    const now = new Date();
    for (let i = 30; i >= 0; i--) {
      const d = new Date(now);
      d.setDate(d.getDate() - i);
      data.push({
        date: d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' }),
        githubSuccess: 80 + Math.random() * 20,
        githubLatency: 100 + Math.random() * 50,
        notionSuccess: 75 + Math.random() * 25,
        notionLatency: 150 + Math.random() * 100,
      });
    }
    return data;
  }, []);

  const handleExportPDF = async () => {
    if (!reportRef.current) return;
    try {
      setIsGenerating(true);
      const canvas = await html2canvas(reportRef.current, { scale: 2, backgroundColor: '#090b14' });
      const imgData = canvas.toDataURL('image/png');
      const pdf = new jsPDF('p', 'mm', 'a4');
      const pdfWidth = pdf.internal.pageSize.getWidth();
      const pdfHeight = (canvas.height * pdfWidth) / canvas.width;
      
      pdf.addImage(imgData, 'PNG', 0, 0, pdfWidth, pdfHeight);
      pdf.save('A-TownChain_Architecture_Report.pdf');
    } catch (err) {
      console.error("Failed to generate PDF", err);
    } finally {
      setIsGenerating(false);
    }
  };

  const allAudits = [
    ...ARCHITECTURE_REVIEW.items,
    ...REPOSITORY_REVIEW.items,
    ...ENTERPRISE_GAP_ANALYSIS.items
  ];
  
  const gapAudits = allAudits.filter(a => a.status === 'Fail' || a.status === 'Warn');

  return (
    <div className="flex flex-col gap-8 p-8 h-full custom-scrollbar overflow-y-auto">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-mono text-white flex items-center gap-2">
            <FileText className="w-6 h-6 text-atc-cyan" />
            System Architecture Reports
          </h1>
          <p className="text-slate-400 mt-2">Export comprehensive architectural data and status.</p>
        </div>
        <button
          onClick={handleExportPDF}
          disabled={isGenerating}
          className="flex items-center gap-2 px-5 py-2.5 bg-atc-cyan/10 hover:bg-atc-cyan/20 text-atc-cyan font-mono text-sm tracking-wider uppercase border border-atc-cyan/30 rounded-xl transition-colors disabled:opacity-50"
        >
          {isGenerating ? <Activity className="w-4 h-4 animate-spin" /> : <Download className="w-4 h-4" />}
          {isGenerating ? 'Generating...' : 'Export as PDF'}
        </button>
      </div>

      {/* The Printable Report Container */}
      <div className="w-full max-w-[800px] mx-auto bg-[#090b14] border border-atc-border/50 rounded-xl p-0 overflow-hidden shadow-2xl relative">
        <div ref={reportRef} className="p-10 bg-[#090b14] text-slate-300">
          
          {/* Document Header */}
          <div className="flex justify-between items-start border-b border-white/10 pb-8 mb-8">
            <div>
              <div className="text-2xl font-bold text-white tracking-widest uppercase mb-1">A-TownChain</div>
              <div className="text-atc-cyan font-mono tracking-widest text-sm">System Architecture Report</div>
            </div>
            <div className="text-right text-xs font-mono text-slate-500">
              <div>Date: {new Date().toLocaleDateString()}</div>
              <div>Report ID: ATC-REP-{Math.floor(Math.random()*10000)}</div>
            </div>
          </div>

          {/* Section 1: System Metrics */}
          <div className="mb-10">
            <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2 border-b border-white/5 pb-2">
              <Activity className="w-5 h-5 text-emerald-400" />
              Current System Metrics
            </h2>
            <div className="grid grid-cols-2 gap-4">
              <div className="p-4 bg-white/5 rounded-lg border border-white/5">
                <div className="text-xs font-mono text-slate-400 uppercase tracking-wider mb-1">Hauptsysteme</div>
                <div className="text-xl font-bold text-white text-emerald-400">{TARGET_STATE.mainSystems.current} / {TARGET_STATE.mainSystems.vision}</div>
              </div>
              <div className="p-4 bg-white/5 rounded-lg border border-white/5">
                <div className="text-xs font-mono text-slate-400 uppercase tracking-wider mb-1">Subsysteme</div>
                <div className="text-xl font-bold text-white text-emerald-400">{TARGET_STATE.subsystems.current} / {TARGET_STATE.subsystems.vision}</div>
              </div>
              <div className="p-4 bg-white/5 rounded-lg border border-white/5">
                <div className="text-xs font-mono text-slate-400 uppercase tracking-wider mb-1">Kernkomponenten</div>
                <div className="text-xl font-bold text-white text-emerald-400">{TARGET_STATE.coreComponents.current} / {TARGET_STATE.coreComponents.vision}</div>
              </div>
              <div className="p-4 bg-white/5 rounded-lg border border-white/5">
                <div className="text-xs font-mono text-slate-400 uppercase tracking-wider mb-1">Services</div>
                <div className="text-xl font-bold text-atc-cyan">{TARGET_STATE.services.current} / {TARGET_STATE.services.vision}</div>
              </div>
            </div>
          </div>

          {/* Section 2: Tier Classifications */}
          <div className="mb-10">
            <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2 border-b border-white/5 pb-2">
              <Layers className="w-5 h-5 text-indigo-400" />
              Tier Classifications
            </h2>
            <div className="space-y-4">
              {TIERS.slice(0, 3).map((tier, idx) => (
                <div key={idx} className="p-4 bg-white/[0.02] rounded-lg border border-white/5">
                  <div className="font-bold text-white mb-1">{tier.name} <span className="opacity-50 font-mono text-xs ml-2">Tier {tier.id}</span></div>
                  <div className="text-sm text-slate-400">{tier.description}</div>
                </div>
              ))}
            </div>
          </div>

          {/* Section 3: Identified Gaps */}
          <div>
            <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2 border-b border-white/5 pb-2">
              <AlertCircle className="w-5 h-5 text-red-400" />
              Identified Compliance Gaps
            </h2>
            {gapAudits.length > 0 ? (
              <div className="space-y-3">
                {gapAudits.map((gap, i) => (
                  <div key={i} className="flex items-start gap-3 p-3 bg-red-500/5 border border-red-500/10 rounded-lg">
                    <AlertCircle className={`w-4 h-4 mt-0.5 shrink-0 ${gap.status === 'Fail' ? 'text-red-400' : 'text-amber-400'}`} />
                    <div>
                      <div className="font-bold text-slate-200 text-sm">{gap.label}</div>
                      <div className="text-xs text-slate-400 mt-1">{gap.description}</div>
                      <div className="text-xs font-mono uppercase mt-2 opacity-50 tracking-wider">Severity: {gap.status}</div>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-4 bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-sm font-mono rounded-lg">
                No active gaps recorded in the audit database. Systems are optimal.
              </div>
            )}
          </div>
          {/* Section 4: Sync Metrics */}
          <div className="mt-10">
            <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2 border-b border-white/5 pb-2">
              <RefreshCw className="w-5 h-5 text-indigo-400" />
              GitHub & Notion Sync Metrics (30 Days)
            </h2>
            <div className="bg-[#0b0e1a] border border-white/5 p-6 rounded-xl">
              <div className="h-64 mb-6">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart data={syncData} margin={{ top: 5, right: 20, bottom: 5, left: 0 }}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" />
                    <XAxis dataKey="date" stroke="#888" fontSize={10} tickMargin={10} />
                    <YAxis stroke="#888" fontSize={10} />
                    <Tooltip contentStyle={{ backgroundColor: '#090b14', borderColor: '#ffffff20', fontSize: '12px' }} />
                    <Legend wrapperStyle={{ fontSize: '12px' }} />
                    <Line type="monotone" name="GitHub Success (%)" dataKey="githubSuccess" stroke="#10b981" strokeWidth={2} dot={false} />
                    <Line type="monotone" name="Notion Success (%)" dataKey="notionSuccess" stroke="#3b82f6" strokeWidth={2} dot={false} />
                  </LineChart>
                </ResponsiveContainer>
              </div>
              <div className="h-64">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart data={syncData} margin={{ top: 5, right: 20, bottom: 5, left: 0 }}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" />
                    <XAxis dataKey="date" stroke="#888" fontSize={10} tickMargin={10} />
                    <YAxis stroke="#888" fontSize={10} />
                    <Tooltip contentStyle={{ backgroundColor: '#090b14', borderColor: '#ffffff20', fontSize: '12px' }} />
                    <Legend wrapperStyle={{ fontSize: '12px' }} />
                    <Line type="monotone" name="GitHub Latency (ms)" dataKey="githubLatency" stroke="#f59e0b" strokeWidth={2} dot={false} />
                    <Line type="monotone" name="Notion Latency (ms)" dataKey="notionLatency" stroke="#ef4444" strokeWidth={2} dot={false} />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </div>
          </div>
          
        </div>
      </div>
    </div>
  );
}
