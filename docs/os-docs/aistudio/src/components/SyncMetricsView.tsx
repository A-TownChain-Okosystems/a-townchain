import React from 'react';
import { useSyncMetrics } from '../contexts/SyncMetricsContext';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
  Legend
} from 'recharts';
import { Activity, Clock, CheckCircle2, XCircle } from 'lucide-react';

export function SyncMetricsView() {
  const { syncHistory, isSyncing, lastSyncTime } = useSyncMetrics();

  // Calculate success rate
  const totalSyncs = syncHistory.length;
  const successfulSyncs = syncHistory.filter(s => s.status === 'success').length;
  const successRate = totalSyncs > 0 ? ((successfulSyncs / totalSyncs) * 100).toFixed(1) : 0;

  // Chart data formatting
  const chartData = syncHistory.map((item, index) => {
    const d = new Date(item.timestamp);
    return {
      index,
      time: d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' }),
      latency: item.latency,
      status: item.status,
      successNum: item.status === 'success' ? 1 : 0,
      errorNum: item.status === 'error' ? 1 : 0,
    };
  });

  return (
    <div className="w-full h-full bg-[#0a0d16] text-white p-6 font-sans flex flex-col gap-6 overflow-y-auto">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-slate-100 flex items-center gap-3">
          <Activity className="w-6 h-6 text-emerald-400" />
          Sync Metrics Dashboard
        </h2>
        <div className="flex items-center gap-4 text-sm font-mono">
          <div className="flex items-center gap-2">
            <span className="text-slate-400">Status:</span>
            {isSyncing ? (
              <span className="text-emerald-400 animate-pulse flex items-center gap-1">
                <span className="w-2 h-2 rounded-full bg-emerald-400"></span> SYNCING
              </span>
            ) : (
              <span className="text-slate-300">IDLE</span>
            )}
          </div>
          <div className="flex items-center gap-2">
            <span className="text-slate-400">Last Sync:</span>
            <span className="text-slate-200">
              {lastSyncTime ? lastSyncTime.toLocaleTimeString() : 'N/A'}
            </span>
          </div>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-[#111625] border border-white/10 rounded-xl p-4 flex items-center gap-4">
          <div className="w-12 h-12 rounded-full bg-indigo-500/20 flex items-center justify-center">
            <Activity className="w-6 h-6 text-indigo-400" />
          </div>
          <div>
            <p className="text-slate-400 text-sm">Total Syncs</p>
            <p className="text-2xl font-bold">{totalSyncs}</p>
          </div>
        </div>
        
        <div className="bg-[#111625] border border-white/10 rounded-xl p-4 flex items-center gap-4">
          <div className="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center">
            <CheckCircle2 className="w-6 h-6 text-emerald-400" />
          </div>
          <div>
            <p className="text-slate-400 text-sm">Success Rate</p>
            <p className="text-2xl font-bold">{successRate}%</p>
          </div>
        </div>

        <div className="bg-[#111625] border border-white/10 rounded-xl p-4 flex items-center gap-4">
          <div className="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center">
            <Clock className="w-6 h-6 text-emerald-400" />
          </div>
          <div>
            <p className="text-slate-400 text-sm">Avg Latency</p>
            <p className="text-2xl font-bold">
              {totalSyncs > 0 
                ? Math.round(syncHistory.reduce((acc, curr) => acc + curr.latency, 0) / totalSyncs) 
                : 0} ms
            </p>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 flex-1 min-h-[300px]">
        {/* Latency Line Chart */}
        <div className="bg-[#111625] border border-white/10 rounded-xl p-4 flex flex-col">
          <h3 className="text-slate-300 font-semibold mb-4 text-sm flex items-center gap-2">
            <Clock className="w-4 h-4 text-indigo-400" /> Latency Over Time (ms)
          </h3>
          <div className="flex-1 w-full min-h-[250px]">
            {chartData.length > 0 ? (
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={chartData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
                  <CartesianGrid strokeDasharray="3 3" stroke="#2d3748" vertical={false} />
                  <XAxis dataKey="time" stroke="#718096" fontSize={10} tickMargin={10} />
                  <YAxis stroke="#718096" fontSize={10} />
                  <Tooltip 
                    contentStyle={{ backgroundColor: '#1a202c', border: '1px solid #2d3748', borderRadius: '8px' }}
                    itemStyle={{ color: '#e2e8f0' }}
                  />
                  <Line 
                    type="monotone" 
                    dataKey="latency" 
                    stroke="#818cf8" 
                    strokeWidth={2}
                    dot={{ fill: '#818cf8', r: 3, strokeWidth: 0 }}
                    activeDot={{ r: 5, fill: '#6366f1' }}
                  />
                </LineChart>
              </ResponsiveContainer>
            ) : (
              <div className="w-full h-full flex flex-col items-center justify-center text-slate-500 font-mono text-sm">
                <Activity className="w-8 h-8 mb-2 opacity-50" />
                No sync data available
              </div>
            )}
          </div>
        </div>

        {/* Success/Error Bar Chart */}
        <div className="bg-[#111625] border border-white/10 rounded-xl p-4 flex flex-col">
          <h3 className="text-slate-300 font-semibold mb-4 text-sm flex items-center gap-2">
            <CheckCircle2 className="w-4 h-4 text-emerald-400" /> Success/Error History
          </h3>
          <div className="flex-1 w-full min-h-[250px]">
            {chartData.length > 0 ? (
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={chartData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
                  <CartesianGrid strokeDasharray="3 3" stroke="#2d3748" vertical={false} />
                  <XAxis dataKey="time" stroke="#718096" fontSize={10} tickMargin={10} />
                  <YAxis stroke="#718096" fontSize={10} />
                  <Tooltip 
                    contentStyle={{ backgroundColor: '#1a202c', border: '1px solid #2d3748', borderRadius: '8px' }}
                    itemStyle={{ color: '#e2e8f0' }}
                  />
                  <Legend wrapperStyle={{ fontSize: '12px' }} />
                  <Bar dataKey="successNum" name="Success" stackId="a" fill="#10b981" />
                  <Bar dataKey="errorNum" name="Error" stackId="a" fill="#ef4444" />
                </BarChart>
              </ResponsiveContainer>
            ) : (
              <div className="w-full h-full flex flex-col items-center justify-center text-slate-500 font-mono text-sm">
                <Activity className="w-8 h-8 mb-2 opacity-50" />
                No sync data available
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
