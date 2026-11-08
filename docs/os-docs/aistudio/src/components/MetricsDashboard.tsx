import React from 'react';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { Activity, LayoutGrid } from 'lucide-react';

const generateData = () => {
  const data = [];
  for (let i = 1; i <= 30; i++) {
    data.push({
      layer: `L${i}`,
      fullLayerName: `Architecture Layer ${i}`,
      core: Math.floor(Math.random() * 50) + 10,
      plugins: Math.floor(Math.random() * 30) + 5,
      services: Math.floor(Math.random() * 40) + 10,
      microservices: Math.floor(Math.random() * 20),
    });
  }
  return data;
};

const data = generateData();

const CustomTooltip = ({ active, payload, label }: any) => {
  if (active && payload && payload.length) {
    return (
      <div className="bg-[#0f172a]/90 backdrop-blur-md border border-slate-700/50 p-4 rounded-xl shadow-xl">
        <p className="text-white font-mono font-bold mb-2 border-b border-slate-700 pb-2">{payload[0].payload.fullLayerName}</p>
        {payload.map((entry: any, index: number) => (
          <div key={index} className="flex items-center gap-2 mb-1">
            <span className="w-3 h-3 rounded-full" style={{ backgroundColor: entry.color }}></span>
            <span className="text-slate-300 font-mono text-sm">{entry.name}:</span>
            <span className="text-white font-mono font-bold">{entry.value}</span>
          </div>
        ))}
        <div className="mt-2 pt-2 border-t border-slate-700 flex justify-between">
          <span className="text-slate-400 font-mono text-sm">Total Active:</span>
          <span className="text-white font-mono font-bold">
            {payload.reduce((sum: number, entry: any) => sum + entry.value, 0)}
          </span>
        </div>
      </div>
    );
  }
  return null;
};

export function MetricsDashboard() {
  return (
    <div className="flex flex-col gap-6 w-full">
      <div className="bg-[#090b14]/60 border border-slate-800/60 rounded-2xl p-6 shadow-2xl relative overflow-hidden">
        <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-emerald-500 via-cyan-500 to-indigo-500 opacity-50" />
        
        <div className="flex items-center gap-4 mb-8">
          <div className="w-12 h-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-[0_0_15px_rgba(6,182,212,0.2)]">
            <LayoutGrid className="w-6 h-6" />
          </div>
          <div>
            <h3 className="text-2xl font-bold tracking-tight text-white mb-1">Component Distribution matrix</h3>
            <p className="text-sm font-mono text-slate-400 flex items-center gap-2">
              <Activity className="w-3 h-3 text-emerald-400" />
              Live distribution of active core components across all 30 architectural layers
            </p>
          </div>
        </div>

        <div className="h-[500px] w-full">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart
              data={data}
              margin={{
                top: 20,
                right: 30,
                left: 0,
                bottom: 5,
              }}
            >
              <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} />
              <XAxis 
                dataKey="layer" 
                stroke="#64748b" 
                tick={{ fill: '#64748b', fontSize: 12, fontFamily: 'monospace' }} 
                tickLine={false}
                axisLine={{ stroke: '#334155' }}
              />
              <YAxis 
                stroke="#64748b" 
                tick={{ fill: '#64748b', fontSize: 12, fontFamily: 'monospace' }} 
                tickLine={false}
                axisLine={false}
              />
              <Tooltip content={<CustomTooltip />} cursor={{ fill: 'rgba(255,255,255,0.05)' }} />
              <Legend 
                wrapperStyle={{ paddingTop: '20px', fontFamily: 'monospace', fontSize: '12px', color: '#cbd5e1' }}
                iconType="circle"
              />
              <Bar dataKey="core" name="Core Modules" stackId="a" fill="#3b82f6" radius={[0, 0, 4, 4]} />
              <Bar dataKey="services" name="Services" stackId="a" fill="#10b981" />
              <Bar dataKey="plugins" name="Plugins" stackId="a" fill="#8b5cf6" />
              <Bar dataKey="microservices" name="Microservices" stackId="a" fill="#ec4899" radius={[4, 4, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  );
}
