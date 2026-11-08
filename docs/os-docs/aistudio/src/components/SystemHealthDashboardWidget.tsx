import React, { useState, useEffect } from 'react';
import { PieChart, Pie, Cell, ResponsiveContainer } from 'recharts';
import { TooltipIcon } from './TooltipIcon';

// Mocking "internal metrics store"
const fetchInternalHealthMetric = () => {
  if (!(window as any)._atc_metrics_store) {
    (window as any)._atc_metrics_store = { systemHealth: 98.4 };
  } else {
    // Add some realistic fluctuation to the metric store
    (window as any)._atc_metrics_store.systemHealth = Math.min(100, Math.max(85, (window as any)._atc_metrics_store.systemHealth + (Math.random() * 2 - 0.7)));
  }
  return (window as any)._atc_metrics_store.systemHealth;
};

export function SystemHealthDashboardWidget() {
  const [health, setHealth] = useState<number>(() => fetchInternalHealthMetric());

  useEffect(() => {
    const interval = setInterval(() => {
      setHealth(fetchInternalHealthMetric());
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  const data = [
    { name: 'Healthy', value: health },
    { name: 'Degraded', value: 100 - health },
  ];

  const color = health > 95 ? '#10b981' : health > 85 ? '#f59e0b' : '#ef4444';
  const COLORS = [color, 'rgba(255, 255, 255, 0.1)'];

  return (
    <div className="flex items-center gap-1 group relative cursor-pointer" title={`Total System Health: ${health.toFixed(1)}%`}>
       <div className="w-8 h-8 relative hidden sm:block">
         <ResponsiveContainer width="100%" height="100%">
           <PieChart>
             <Pie
               data={data}
               cx="50%"
               cy="50%"
               startAngle={225}
               endAngle={-45}
               innerRadius="70%"
               outerRadius="100%"
               stroke="none"
               dataKey="value"
               isAnimationActive={false}
             >
               {data.map((entry, index) => (
                 <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
               ))}
             </Pie>
           </PieChart>
         </ResponsiveContainer>
         <div className="absolute inset-0 flex items-center justify-center font-mono text-[9px] font-bold text-white z-10 pointer-events-none">
           {Math.floor(health)}
         </div>
       </div>
    </div>
  );
}
