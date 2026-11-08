import React, { useState } from 'react';
import { Play, RotateCcw } from 'lucide-react';

export function JsExampleRunner({ type }: { type: 'network' | 'nodes' }) {
  const [output, setOutput] = useState<string | null>(null);
  const [isRunning, setIsRunning] = useState(false);

  const codeNetwork = `fetch("https://api.a-townchain.network/v1/chain/status", {
  method: "GET",
  headers: {
    "Authorization": "Bearer <YOUR_TOKEN>"
  }
})
.then(response => response.json())
.then(data => console.log("Network Status:", data))
.catch(error => console.error("Error:", error));`;

  const codeNodes = `async function getNodeInfo() {
  try {
    const response = await fetch("https://api.a-townchain.network/v1/nodes", {
      headers: { "Content-Type": "application/json" }
    });
    const nodes = await response.json();
    console.log(\`Zahl der aktiven Nodes: \${nodes.activeCount}\`, nodes.list);
  } catch (err) {
    console.error("Fetch misslungen:", err);
  }
}

getNodeInfo();`;

  const handleRun = () => {
    setIsRunning(true);
    setOutput(null);

    // Simulate API delay
    setTimeout(() => {
      if (type === 'network') {
        setOutput(`> Network Status: {
  status: 'online',
  currentBlock: 8593021,
  network: 'mainnet',
  atvVersion: '2.1.4'
}`);
      } else {
        setOutput(`> Zahl der aktiven Nodes: 3
> [
  { id: 'master-1', region: 'EU', latency: '12ms' },
  { id: 'rep-east', region: 'US', latency: '34ms' },
  { id: 'edge-ap', region: 'AP', latency: '42ms' }
]`);
      }
      setIsRunning(false);
    }, 600);
  };

  return (
    <div className="mt-4 rounded-xl border border-white/10 bg-[#06080c] overflow-hidden">
      <div className="bg-[#090b14] px-4 py-2 flex items-center justify-between border-b border-white/10">
        <span className="text-xs font-mono text-slate-400">interactive.js</span>
        <button 
          onClick={handleRun}
          disabled={isRunning}
          className="flex items-center gap-1.5 px-3 py-1 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/20 rounded text-xs transition-colors disabled:opacity-50"
        >
          {isRunning ? <RotateCcw className="w-3 h-3 animate-spin" /> : <Play className="w-3 h-3" />}
          {isRunning ? 'Running...' : 'Run Code'}
        </button>
      </div>
      <div className="p-4 overflow-x-auto text-xs font-mono">
        <pre className="text-slate-300">
          <code>{type === 'network' ? codeNetwork : codeNodes}</code>
        </pre>
      </div>
      {(output || isRunning) && (
        <div className="border-t border-white/10 p-4 bg-black/60 font-mono text-xs">
          {isRunning ? (
            <span className="text-slate-500 italic">Executing script... awaiting console output...</span>
          ) : (
            <pre className="text-atc-cyan whitespace-pre-wrap">{output}</pre>
          )}
        </div>
      )}
    </div>
  );
}
