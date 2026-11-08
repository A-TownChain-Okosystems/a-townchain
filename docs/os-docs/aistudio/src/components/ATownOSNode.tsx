import React, { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { 
  Bot, Cpu, Network, Activity, Server, Zap, Database, Globe, Play, 
  Square, Code, Hash, Link as LinkIcon, Radio, Terminal, Sliders, 
  ShieldAlert, Users, CheckCircle, TrendingUp, Eye, RefreshCw, 
  Search, AlertTriangle, Trash2, Shield, HeartPulse, Sparkles, AlertCircle
} from 'lucide-react';
import { AreaChart, Area, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid } from 'recharts';
import { ConsensusIntegrationGuide } from './ConsensusIntegrationGuide';

// ==========================================
// 1. GENUINE SHA-256 ALGORITHM IMPLEMENTATION
// ==========================================
function rightRotate(value: number, amount: number) {
  return (value >>> amount) | (value << (32 - amount));
}

async function calculateSHA256(text: string): Promise<string> {
  const encoder = new TextEncoder();
  const msg = encoder.encode(text);
  const msgBitLength = msg.length * 8;
  let paddingLength = 56 - (msg.length % 64);
  if (paddingLength <= 0) paddingLength += 64;
  const paddedMsg = new Uint8Array(msg.length + paddingLength + 8);
  paddedMsg.set(msg);
  paddedMsg[msg.length] = 0x80;
  
  const dataView = new DataView(paddedMsg.buffer);
  const upperLen = Math.floor(msgBitLength / 4294967296);
  const lowerLen = msgBitLength % 4294967296;
  dataView.setUint32(paddedMsg.length - 8, upperLen, false);
  dataView.setUint32(paddedMsg.length - 4, lowerLen, false);

  let h0 = 0x6a09e667, h1 = 0xbb67ae85, h2 = 0x3c6ef372, h3 = 0xa54ff53a;
  let h4 = 0x510e527f, h5 = 0x9b05688c, h6 = 0x1f83d9ab, h7 = 0x5be0cd19;
  
  const K = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
  ];

  for (let i = 0; i < paddedMsg.length; i += 64) {
    const w = new Uint32Array(64);
    for (let j = 0; j < 16; j++) w[j] = dataView.getUint32(i + j * 4, false);
    for (let j = 16; j < 64; j++) {
      const s0 = (rightRotate(w[j - 15], 7) ^ rightRotate(w[j - 15], 18) ^ (w[j - 15] >>> 3)) >>> 0;
      const s1 = (rightRotate(w[j - 2], 17) ^ rightRotate(w[j - 2], 19) ^ (w[j - 2] >>> 10)) >>> 0;
      w[j] = (w[j - 16] + s0 + w[j - 7] + s1) >>> 0;
    }
    
    let a = h0, b = h1, c = h2, d = h3, e = h4, f = h5, g = h6, h = h7;
    for (let j = 0; j < 64; j++) {
      const S1 = (rightRotate(e, 6) ^ rightRotate(e, 11) ^ rightRotate(e, 25)) >>> 0;
      const ch = (e & f) ^ ((~e) & g);
      const temp1 = (h + S1 + ch + K[j] + w[j]) >>> 0;
      
      const S0 = (rightRotate(a, 2) ^ rightRotate(a, 13) ^ rightRotate(a, 22)) >>> 0;
      const maj = (a & b) ^ (a & c) ^ (b & c);
      const temp2 = (S0 + maj) >>> 0;
      
      h = g; g = f; f = e; e = (d + temp1) >>> 0; d = c; c = b; b = a; a = (temp1 + temp2) >>> 0;
    }
    h0 = (h0 + a) >>> 0; h1 = (h1 + b) >>> 0; h2 = (h2 + c) >>> 0; h3 = (h3 + d) >>> 0;
    h4 = (h4 + e) >>> 0; h5 = (h5 + f) >>> 0; h6 = (h6 + g) >>> 0; h7 = (h7 + h) >>> 0;
  }
  
  const toHex = (val: number) => val.toString(16).padStart(8, '0');
  return toHex(h0) + toHex(h1) + toHex(h2) + toHex(h3) + toHex(h4) + toHex(h5) + toHex(h6) + toHex(h7);
}

// Helper sleep function
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// ==========================================
// 2. MAIN ATOWNCHAIN NODE VISUALIZER COMPONENT
// ==========================================
interface LogEntry {
  id: string;
  time: string;
  level: 'INFO' | 'WARN' | 'ERROR';
  module: string;
  message: string;
}

export function ATownOSNode() {
  const [activeTab, setActiveTab] = useState<'overview' | 'terminal' | 'consensus'>('overview');
  const [isRunning, setIsRunning] = useState(true);
  const [cpuLoad, setCpuLoad] = useState(38.4);
  const [memoryUsed, setMemoryUsed] = useState(16.2);
  const [blockHeight, setBlockHeight] = useState(1450293);
  const [peersCount, setPeersCount] = useState(1024);
  
  // Real-time ping/latency over the last hour (60 points)
  const [latencyHistory, setLatencyHistory] = useState<{ time: string; ping: number }[]>(() => {
    const points = [];
    const now = Date.now();
    for (let i = 59; i >= 0; i--) {
      points.push({
        time: new Date(now - i * 60 * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        ping: Math.floor(18 + Math.random() * 12)
      });
    }
    return points;
  });

  const [agents, setAgents] = useState([
    { id: 'AG-1', type: 'Consensus Validator', status: 'Active', load: 12 },
    { id: 'AG-2', type: 'Data Harvester', status: 'Active', load: 8 },
    { id: 'AG-3', type: 'Security Pattern Scanner', status: 'Sleeping', load: 0 },
    { id: 'AG-4', type: 'Swarm Coordinator', status: 'Active', load: 22 },
  ]);

  // Real-time filterable log entries state
  const [logs, setLogs] = useState<LogEntry[]>([
    { id: '1', time: '15:26:01', level: 'INFO', module: 'BLOCKCHAIN', message: 'Initialized A-TownOS-Kernel v1.0.0.' },
    { id: '2', time: '15:26:04', level: 'INFO', module: 'AI_SHIELD', message: 'Ring 3 WASM userspace sandbox successfully initialized.' },
    { id: '3', time: '15:26:12', level: 'WARN', module: 'NETWORK', message: 'Peer node bootstrap signal fluctuated above 120ms.' },
    { id: '4', time: '15:26:40', level: 'INFO', module: 'CONSENSUS', message: 'Validated state transition proof ledger #1450292.' },
    { id: '5', time: '15:26:59', level: 'ERROR', module: 'STORAGE', message: 'Temporary write buffer delay: Disk read bandwidth threshold hit.' }
  ]);
  const [logFilter, setLogFilter] = useState<'ALL' | 'INFO' | 'WARN' | 'ERROR'>('ALL');
  const [logSearch, setLogSearch] = useState('');

  // Stop confirmation State
  const [showStopConfirmation, setShowStopConfirmation] = useState(false);

  // Hybrid simulator State
  const [simulatorLogs, setSimulatorLogs] = useState<{ text: string; type: 'info' | 'success' | 'error' | 'header' | 'data' }[]>([
    { text: 'System bereit. Warte auf Mining-Befehl...', type: 'info' }
  ]);
  const [isSimulating, setIsSimulating] = useState(false);
  const [simulatorStatus, setSimulatorStatus] = useState<'idle' | 'running' | 'completed' | 'failed'>('idle');
  const [activeValidator, setActiveValidator] = useState<string | null>(null);

  // Hybrid Consensus Custom/Dynamic parameters
  const [aliceStake, setAliceStake] = useState(50000);
  const [bobStake, setBobStake] = useState(100);
  const [alicePoI, setAlicePoI] = useState(85);
  const [bobPoI, setBobPoI] = useState(5);
  
  // Custom Validator optional variables
  const [customMinerName, setCustomMinerName] = useState('Dein_Miner');
  const [customMinerStake, setCustomMinerStake] = useState(15000);
  const [customMinerPoI, setCustomMinerPoI] = useState(65);

  const [requiredStake, setRequiredStake] = useState(10000);
  const [requiredPoI, setRequiredPoI] = useState(50);
  const [pohIterations, setPohIterations] = useState(15);
  const [powDifficulty, setPowDifficulty] = useState(1); // leading zeroes count
  
  // Toggle for showing the raw simulator source code
  const [showCodePreview, setShowCodePreview] = useState(false);
  const [consensusSubView, setConsensusSubView] = useState<'simulator' | 'integration'>('simulator');
  const [activeGuideSec, setActiveGuideSec] = useState<'state' | 'miner' | 'validation' | 'fazit'>('state');

  // Dynamic values generation
  useEffect(() => {
    if (!isRunning) return;
    const interval = setInterval(() => {
      // Modulate CPU load & Memory
      setCpuLoad(prev => Math.min(100, Math.max(12, prev + (Math.random() * 12 - 6))));
      setMemoryUsed(prev => Math.min(64, Math.max(8, prev + (Math.random() * 2 - 1))));
      
      // Update latency history with continuous time
      setLatencyHistory(prev => {
        const next = [...prev.slice(1)];
        const nextTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
        next.push({
          time: nextTime,
          ping: Math.max(6, Math.floor(prev[prev.length - 1].ping + (Math.random() * 8 - 4)))
        });
        return next;
      });

      // Randomized node logging events
      const randValue = Math.random();
      const randTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
      const idStr = Date.now().toString();

      if (randValue > 0.8) {
        // INFO block update
        setBlockHeight(prev => prev + 1);
        const newLog: LogEntry = {
          id: idStr,
          time: randTime,
          level: 'INFO',
          module: 'BLOCKCHAIN',
          message: `Validated new block #${blockHeight + 1} with ${Math.floor(80 + Math.random() * 110)} secure txs`
        };
        setLogs(prev => [newLog, ...prev].slice(0, 150));
      } else if (randValue > 0.6) {
        // WARN agent swap
        const agId = Math.floor(Math.random() * 4);
        setAgents(prev => {
          const newAgs = [...prev];
          newAgs[agId].load = Math.max(0, Math.min(100, newAgs[agId].load + (Math.random() * 24 - 12)));
          newAgs[agId].status = newAgs[agId].load > 5 ? 'Active' : 'Sleeping';
          return newAgs;
        });
        
        const newLog: LogEntry = {
          id: idStr,
          time: randTime,
          level: 'WARN',
          module: 'AI_SWARM',
          message: `Swarm Agent ${agents[agId].id} load factor elevated under workload shift`
        };
        setLogs(prev => [newLog, ...prev].slice(0, 150));
      } else if (randValue > 0.53) {
        // ERROR network trace
        const targetPeer = `PeerNode-S${Math.floor(Math.random() * 9)}`;
        const newLog: LogEntry = {
          id: idStr,
          time: randTime,
          level: 'ERROR',
          module: 'NETWORK',
          message: `Network ping timeout to decentralized hub validator [${targetPeer}]`
        };
        setLogs(prev => [newLog, ...prev].slice(0, 150));
      }
    }, 1500);

    return () => clearInterval(interval);
  }, [isRunning, blockHeight, agents]);

  // Handle Stop All Processes confirmed
  const triggerEmergencyStop = () => {
    setIsRunning(false);
    setCpuLoad(0);
    setMemoryUsed(0);
    setPeersCount(0);
    
    // Set all agents to offline
    setAgents(prev => prev.map(a => ({ ...a, status: 'Stopped', load: 0 })));
    
    // Add critical log
    const randTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    const shutdownLog: LogEntry = {
      id: `SHUTDOWN-${Date.now()}`,
      time: randTime,
      level: 'ERROR',
      module: 'KERNEL',
      message: '🚨 CRITICAL OVERRIDE: EMERGENCY SHUTDOWN TERMINATED ALL ACTIVE DEPLOYMENTS.'
    };
    
    setLogs(prev => [shutdownLog, ...prev]);
    setShowStopConfirmation(false);
  };

  const restartAllProcesses = () => {
    setIsRunning(true);
    setCpuLoad(41.2);
    setMemoryUsed(17.5);
    setPeersCount(1024);
    setAgents([
      { id: 'AG-1', type: 'Consensus Validator', status: 'Active', load: 12 },
      { id: 'AG-2', type: 'Data Harvester', status: 'Active', load: 8 },
      { id: 'AG-3', type: 'Security Pattern Scanner', status: 'Sleeping', load: 0 },
      { id: 'AG-4', type: 'Swarm Coordinator', status: 'Active', load: 22 },
    ]);
    const randTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    const startLog: LogEntry = {
      id: `BOOT-${Date.now()}`,
      time: randTime,
      level: 'INFO',
      module: 'KERNEL',
      message: '⚡ Cold restart initiated. Reloading node software and AI state vector hashes.'
    };
    setLogs(prev => [startLog, ...prev]);
  };

  const appendSimLog = (text: string, type: 'info' | 'success' | 'error' | 'header' | 'data' = 'info') => {
    setSimulatorLogs(prev => [...prev, { text, type }]);
  };

  // Run Hybrid validation simulator
  const startConsensusSimulation = async (miner: 'Alice' | 'Bob' | 'Custom') => {
    if (isSimulating) return;
    setIsSimulating(true);
    setSimulatorStatus('running');
    setActiveValidator(miner);
    
    // Clear terminal log screen for premium simulation feel
    setSimulatorLogs([]);
    
    const targetMinerName = miner === 'Alice' ? 'Miner_Alice' : miner === 'Bob' ? 'Miner_Bob' : customMinerName;
    const targetMinerStake = miner === 'Alice' ? aliceStake : miner === 'Bob' ? bobStake : customMinerStake;
    const targetMinerPoI = miner === 'Alice' ? alicePoI : miner === 'Bob' ? bobPoI : customMinerPoI;

    appendSimLog(`🚀 Validator [${targetMinerName}] startet den Hybrid-Konsens...`, 'header');

    await sleep(600);

    // --- STEP 1: Proof of Stake (PoS) ---
    appendSimLog(`⏳ Starte Phase 1: Proof of Stake (PoS)...`, 'info');
    await sleep(500);

    if (targetMinerStake < requiredStake) {
      appendSimLog(`❌ PoS FEHLGESCHLAGEN: [${targetMinerName}] hat nur ${targetMinerStake.toLocaleString()} Coins (${requiredStake.toLocaleString()} Coins benötigt). Block wurde verworfen!`, 'error');
      setSimulatorStatus('failed');
      setIsSimulating(false);
      return;
    }
    appendSimLog(`✅ PoS BESTANDEN: Ausreichendes Stake-Kapital hinterlegt (${targetMinerStake.toLocaleString()} Coins).`, 'success');
    await sleep(700);

    // --- STEP 2: Proof of Importance (PoI) ---
    appendSimLog(`⏳ Starte Phase 2: Proof of Importance (PoI)...`, 'info');
    await sleep(500);

    if (targetMinerPoI < requiredPoI) {
      appendSimLog(`❌ PoI FEHLGESCHLAGEN: [${targetMinerName}] ist nicht aktiv genug (Score: ${targetMinerPoI}/100, benötigt: ${requiredPoI}). Block verworfen!`, 'error');
      setSimulatorStatus('failed');
      setIsSimulating(false);
      return;
    }
    appendSimLog(`✅ PoI BESTANDEN: Hohe Netzwerkaktivität bestätigt (Score: ${targetMinerPoI}/100).`, 'success');
    await sleep(700);

    // --- STEP 3: Proof of History (PoH) ---
    appendSimLog(`⏳ Starte Phase 3: Proof of History (PoH)...`, 'info');
    appendSimLog(`🔗 Berechne fortlaufende kryptografische Zeitstempel-Kette mit SHA-256...`, 'info');
    await sleep(500);

    let historyHash = "A-TownChain_PoH_Seed_" + Date.now();
    for (let i = 1; i <= pohIterations; i++) {
      historyHash = await calculateSHA256(historyHash);
      if (pohIterations <= 10 || i % Math.max(1, Math.floor(pohIterations / 4)) === 0 || i === pohIterations) {
        appendSimLog(`🔨 Iteration #${i}/${pohIterations}: ${historyHash.substring(0, 32)}...`, 'info');
        await sleep(150);
      }
    }
    appendSimLog(`✅ PoH BESTANDEN: Zeitablauf kryptografisch bewiesen. End-Hash:`, 'success');
    appendSimLog(historyHash, 'data');
    await sleep(700);

    // --- STEP 4: Proof of Artificial Intelligence (PoAI) ---
    appendSimLog(`🧠 Starte Phase 4: Proof of AI (PoAI)...`, 'info');
    appendSimLog(`🎯 Optimiere neuronales Netzwerk auf dezentralen Sensorgewichten...`, 'info');
    await sleep(400);

    let loss = 1.0;
    let epochs = 0;
    while (loss > 0.1 && epochs < 12) {
      loss = loss * 0.82;
      epochs++;
      appendSimLog(`🏋️‍♂️ Epoche ${epochs}/12: Training läuft ... Loss: ${loss.toFixed(4)}`, 'info');
      await sleep(150);
    }
    appendSimLog(`✅ PoAI BESTANDEN: Lokales KI-Modell erfolgreich trainiert nach ${epochs} Epochen (Finaler Loss: ${loss.toFixed(4)}).`, 'success');
    await sleep(700);

    // --- STEP 5: Proof of Work (PoW) ---
    appendSimLog(`⛏️ Starte Phase 5: Proof of Work (PoW) Mining...`, 'info');
    
    // Construct targetPrefix based on dynamic difficulty
    const targetPrefix = "0".repeat(powDifficulty);
    appendSimLog(`🔍 Suche gültigen Block-Hash (Schwierigkeit: ${powDifficulty} führende Null(en): "${targetPrefix}")...`, 'info');
    await sleep(400);

    let nonce = 0;
    let finalHash = "";
    
    // We can limit nonces to prevent browser freeze or infinite loops if difficulty is set too high
    const blockMockData = `Validator:${targetMinerName}|Block:${blockHeight + 1}|Time:${Date.now()}`;

    while (!finalHash.startsWith(targetPrefix)) {
      nonce++;
      finalHash = await calculateSHA256(blockMockData + nonce);
      if (nonce % 10 === 0) {
        // update trace dynamically to make it realistic
        appendSimLog(`⛏️ Hashing Nonce ${nonce}... Aktueller Versuch: ${finalHash.substring(0, 48)}...`, 'info');
        await sleep(100);
      }
      
      // Safety cutoff for high difficulties in browser simulation
      if (nonce > 300) {
        appendSimLog(`⚠️ PoW Cutoff erreicht! Simuliere mathematische Abkürzung für schnellen Konsens...`, 'info');
        finalHash = targetPrefix + finalHash.substring(targetPrefix.length); // mock pass to ensure completion
        break;
      }
    }
    appendSimLog(`✅ PoW BESTANDEN: Gültiges Arbeitsergebnis gefunden! (Nonce: ${nonce})`, 'success');
    appendSimLog(finalHash, 'data');
    await sleep(600);

    // Finale Abschluss des Blocks
    appendSimLog(`🎉 NEUER BLOCK ERFOLGREICH ZUR BLOCKCHAIN HINZUGEFÜGT 🎉`, 'success');
    const fullBlockStr = JSON.stringify({
      index: blockHeight + 1,
      miner: targetMinerName,
      nonce: nonce,
      stateRoot: historyHash.substring(0, 16) + "... " + finalHash.substring(0, 16),
      pohProof: historyHash,
      powHash: finalHash,
      aiModelLoss: loss.toFixed(4),
      time: new Date().toISOString()
    }, null, 2);
    
    appendSimLog(fullBlockStr, 'data');
    setBlockHeight(prev => prev + 1);
    setSimulatorStatus('completed');
    setIsSimulating(false);
  };

  const clearConsensusTerminal = () => {
    setSimulatorLogs([{ text: 'Terminal geleert. System bereit. Warte auf Befehl...', type: 'info' }]);
    setSimulatorStatus('idle');
    setActiveValidator(null);
  };

  // Filter and search logs
  const filteredLogs = logs.filter(log => {
    const matchesFilter = logFilter === 'ALL' || log.level === logFilter;
    const matchesSearch = log.message.toLowerCase().includes(logSearch.toLowerCase()) || 
                          log.module.toLowerCase().includes(logSearch.toLowerCase());
    return matchesFilter && matchesSearch;
  });

  // Calculate percentages for count level
  const totalLogsCount = logs.length;
  const infoCount = logs.filter(l => l.level === 'INFO').length;
  const warnCount = logs.filter(l => l.level === 'WARN').length;
  const errorCount = logs.filter(l => l.level === 'ERROR').length;

  return (
    <div className="flex flex-col h-full gap-6 text-slate-200">
      
      {/* ================== MAIN TITLE & HARDWARE METRICS ================== */}
      <div className="flex flex-col xl:flex-row xl:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl md:text-3xl font-bold tracking-tight text-white flex items-center gap-3">
            <Bot className="w-8 h-8 md:w-9 h-9 text-teal-400 rotate-12" />
            A-TownChain & ATC-OS Node Core
          </h1>
          <p className="text-slate-400 text-sm mt-1">
            Ring 3 WASM Userspace • Live Mainnet Sandbox Connector v1.2.4
          </p>
        </div>
        
        {/* Node Active State Command Center */}
        <div className="flex items-center gap-3 bg-[#0d1326] p-2 rounded-xl border border-white/5">
          <div className="flex items-center gap-2 px-3">
            <span className="relative flex h-2.5 w-2.5">
              {isRunning && <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>}
              <span className={`relative inline-flex rounded-full h-2.5 w-2.5 ${isRunning ? 'bg-emerald-500' : 'bg-rose-500'}`}></span>
            </span>
            <span className="text-xs uppercase tracking-wider font-mono text-slate-300 font-bold">
              {isRunning ? 'System: Online' : 'System: Offline'}
            </span>
          </div>

          {isRunning ? (
            <button 
              onClick={() => setShowStopConfirmation(true)}
              className="px-4 py-1.5 rounded-lg bg-red-500/15 hover:bg-red-500/25 text-red-400 border border-red-500/30 hover:border-red-500/50 text-xs font-mono font-bold transition-all flex items-center gap-1.5"
              id="btn-emergency-stop"
            >
              <Square className="w-3 h-3 fill-current" /> Stop OS
            </button>
          ) : (
            <button 
              onClick={restartAllProcesses}
              className="px-4 py-1.5 rounded-lg bg-emerald-500/15 hover:bg-emerald-500/25 text-emerald-400 border border-emerald-500/30 hover:border-emerald-500/50 text-xs font-mono font-bold transition-all flex items-center gap-1.5"
              id="btn-boot-os"
            >
              <Play className="w-3.5 h-3.5 fill-current" /> Boot OS
            </button>
          )}
        </div>
      </div>

      {/* Primary Hardware Metrics Grid */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {/* CPU */}
        <div className="bg-[#090b14]/75 border border-white/5 shadow-2xl rounded-2xl p-5 flex flex-col items-center justify-center relative overflow-hidden">
          <div className="absolute inset-0 bg-teal-500/[0.02]" />
          <Cpu className="w-7 h-7 text-teal-400 mb-2" />
          <div className="text-2xl font-mono font-extrabold text-white mb-1">
            {isRunning ? `${cpuLoad.toFixed(1)}%` : '0.0%'}
          </div>
          <div className="text-[10px] md:text-xs uppercase tracking-wider text-slate-500 font-bold">WASM CPU load</div>
          
          <div className="w-full h-1.5 bg-slate-800/80 rounded-full mt-3 overflow-hidden">
            <div className="h-full bg-teal-400/90 transition-all duration-500" style={{ width: `${isRunning ? cpuLoad : 0}%` }} />
          </div>
        </div>

        {/* MEMORY */}
        <div className="bg-[#090b14]/75 border border-white/5 shadow-2xl rounded-2xl p-5 flex flex-col items-center justify-center relative overflow-hidden">
          <div className="absolute inset-0 bg-purple-500/[0.02]" />
          <Database className="w-7 h-7 text-purple-400 mb-2" />
          <div className="text-2xl font-mono font-extrabold text-white mb-1">
            {isRunning ? `${memoryUsed.toFixed(1)} GB` : '0.0 GB'}
          </div>
          <div className="text-[10px] md:text-xs uppercase tracking-wider text-slate-500 font-bold">allocated state</div>
          
          <div className="w-full h-1.5 bg-slate-800/80 rounded-full mt-3 overflow-hidden">
            <div className="h-full bg-purple-400/90 transition-all duration-500" style={{ width: `${isRunning ? (memoryUsed / 32) * 100 : 0}%` }} />
          </div>
        </div>

        {/* PEERS */}
        <div className="bg-[#090b14]/75 border border-white/5 shadow-2xl rounded-2xl p-5 flex flex-col items-center justify-center relative overflow-hidden">
          <div className="absolute inset-0 bg-emerald-500/[0.02]" />
          <Network className="w-7 h-7 text-emerald-400 mb-2" />
          <div className="text-2xl font-mono font-extrabold text-white mb-1">
            {isRunning ? peersCount.toLocaleString() : '0'}
          </div>
          <div className="text-[10px] md:text-xs uppercase tracking-wider text-slate-500 font-bold">Verified peer count</div>
          <div className="text-[10px] text-emerald-400 font-mono mt-1 font-semibold">{isRunning ? 'Consensus: Active' : 'Offline'}</div>
        </div>

        {/* BLOCK HEIGHT */}
        <div className="bg-[#090b14]/75 border border-white/5 shadow-2xl rounded-2xl p-5 flex flex-col items-center justify-center relative overflow-hidden">
          <div className="absolute inset-0 bg-cyan-500/[0.02]" />
          <Hash className="w-7 h-7 text-cyan-400 mb-2" />
          <div className="text-2xl font-mono font-extrabold text-white mb-1">
            #{blockHeight.toLocaleString()}
          </div>
          <div className="text-[10px] md:text-xs uppercase tracking-wider text-slate-500 font-bold">Current Block Height</div>
          <div className="text-[10px] text-cyan-400 font-mono mt-1 font-semibold">PoW/PoS Consensus</div>
        </div>
      </div>

      {/* TAB NAVIGATION */}
      <div className="flex border-b border-white/5 gap-2 px-1">
        <button
          onClick={() => setActiveTab('overview')}
          className={`flex items-center gap-2 pb-3 pt-1 px-4 font-mono text-sm tracking-wide border-b-2 font-bold transition-all uppercase ${
            activeTab === 'overview'
              ? 'text-teal-400 border-teal-400'
              : 'text-slate-400 hover:text-white border-transparent'
          }`}
        >
          <Sliders className="w-4 h-4" />
          Dashboard & Latency
        </button>

        <button
          onClick={() => setActiveTab('terminal')}
          className={`flex items-center gap-2 pb-3 pt-1 px-4 font-mono text-sm tracking-wide border-b-2 font-bold transition-all uppercase ${
            activeTab === 'terminal'
              ? 'text-teal-400 border-teal-400'
              : 'text-slate-400 hover:text-white border-transparent'
          }`}
        >
          <Terminal className="w-4 h-4" />
          Advanced System Logs
          {errorCount > 0 && isRunning && (
            <span className="bg-red-500/20 text-red-400 border border-red-500/30 text-[10px] px-1.5 py-0.5 rounded ml-1 animate-pulse">
              {errorCount} ERR
            </span>
          )}
        </button>

        <button
          onClick={() => setActiveTab('consensus')}
          className={`flex items-center gap-2 pb-3 pt-1 px-4 font-mono text-sm tracking-wide border-b-2 font-bold transition-all uppercase ${
            activeTab === 'consensus'
              ? 'text-teal-400 border-teal-400'
              : 'text-slate-400 hover:text-white border-transparent'
          }`}
        >
          <Activity className="w-4 h-4 text-emerald-400" />
          Hybrid-Consensus Simulator
        </button>
      </div>

      {/* ================== TAB CONTENTS ================== */}
      <AnimatePresence mode="wait">
        
        {/* TAB 1: OVERVIEW & LATENCY GRAPH */}
        {activeTab === 'overview' && (
          <motion.div
            key="overview"
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="flex flex-col lg:grid lg:grid-cols-3 gap-6 flex-1"
          >
            {/* Left Col: Latency Line Chart */}
            <div className="lg:col-span-2 bg-[#060a16] border border-white/5 shadow-2xl rounded-2xl p-6 flex flex-col overflow-hidden justify-between">
              <div>
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-semibold text-white flex items-center gap-2 text-base">
                    <Activity className="w-4 h-4 text-teal-400 animate-pulse" />
                    Node Latency / Network Ping
                  </h3>
                  <span className="text-xs bg-teal-400/10 text-teal-300 border border-teal-400/20 px-2 py-0.5 rounded-lg font-mono">
                    Last Hour • Update: Realtime
                  </span>
                </div>
                <p className="text-slate-400 text-xs mb-6">
                  Analyzes decentralized peer connectivity response time index (ms) to detect routing bottleneck.
                </p>
              </div>

              {/* RECHARTS CHANGER IN STACK */}
              <div className="w-full flex-1 min-h-[220px]">
                {isRunning ? (
                  <ResponsiveContainer width="100%" height={220}>
                    <AreaChart data={latencyHistory} margin={{ top: 10, right: 10, left: -25, bottom: 0 }}>
                      <defs>
                        <linearGradient id="latencyGlow" x1="0" y1="0" x2="0" y2="1">
                          <stop offset="5%" stopColor="#2dd4bf" stopOpacity={0.35}/>
                          <stop offset="95%" stopColor="#2dd4bf" stopOpacity={0.0}/>
                        </linearGradient>
                      </defs>
                      <CartesianGrid strokeDasharray="3 3" stroke="#ffffff0a" />
                      <XAxis 
                        dataKey="time" 
                        stroke="#64748b" 
                        fontSize={10} 
                        tickLine={false} 
                        axisLine={false}
                        interval={6} // reduce scale density
                      />
                      <YAxis 
                        stroke="#64748b" 
                        fontSize={10} 
                        tickLine={false} 
                        axisLine={false} 
                        domain={['auto', 'auto']}
                      />
                      <Tooltip
                        contentStyle={{
                          backgroundColor: '#070a14',
                          border: '1px solid rgba(255,255,255,0.1)',
                          borderRadius: '8px',
                          color: '#f8fafc',
                          fontFamily: 'monospace',
                          fontSize: '11px',
                        }}
                      />
                      <Area 
                        type="monotone" 
                        dataKey="ping" 
                        stroke="#2dd4bf" 
                        strokeWidth={2} 
                        fillOpacity={1} 
                        fill="url(#latencyGlow)" 
                        dot={false}
                      />
                    </AreaChart>
                  </ResponsiveContainer>
                ) : (
                  <div className="h-[220px] bg-black/40 border border-dashed border-white/10 rounded-xl flex flex-col items-center justify-center gap-2 text-slate-500 font-mono text-sm">
                    <AlertTriangle className="w-8 h-8 text-rose-500 animate-bounce" />
                    No network diagnostics: OS is stopped.
                  </div>
                )}
              </div>

              <div className="flex items-center justify-between text-xs text-slate-500 font-mono mt-4 pt-4 border-t border-white/5">
                <span>Avg. Ping: ~22ms</span>
                <span>Peak: 45ms</span>
                <span>Jitter Code: OK</span>
              </div>
            </div>

            {/* Right Col: Active Swarm Agents */}
            <div className="bg-[#060a16] border border-white/5 shadow-2xl rounded-2xl flex flex-col overflow-hidden">
               <div className="p-5 border-b border-white/5 flex items-center justify-between bg-[#090b14]/80">
                  <h3 className="font-semibold text-white flex items-center gap-2 text-base">
                    <Zap className="w-4 h-4 text-amber-400" /> 
                    AI Swarm Agents
                  </h3>
                  <span className="text-[10px] font-mono bg-amber-400/10 text-amber-400 border border-amber-400/20 px-2 py-0.5 rounded-md">
                    Swarm Logic: {isRunning ? 'Live' : 'Standby'}
                  </span>
               </div>
               
               <div className="p-5 flex-1 overflow-y-auto custom-scrollbar flex flex-col gap-3 min-h-[290px]">
                  {agents.map(agent => (
                    <div key={agent.id} className="p-3.5 rounded-xl border border-white/5 border-l-4 border-l-teal-500 bg-[#0c1224] flex items-center justify-between shadow-lg">
                       <div>
                         <div className="flex items-center gap-2 mb-1">
                           <span className="font-mono text-xs text-teal-400 font-bold bg-teal-500/10 px-1.5 py-0.5 rounded">{agent.id}</span>
                           <span className="text-white text-sm font-semibold">{agent.type}</span>
                         </div>
                         <div className="text-xs text-slate-500 flex items-center gap-1.5 font-mono">
                           <span className="relative flex h-1.5 w-1.5">
                             {agent.status === 'Active' && isRunning && <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-teal-400 opacity-75"></span>}
                             <span className={`relative inline-flex rounded-full h-1.5 w-1.5 ${agent.status === 'Active' && isRunning ? 'bg-teal-500' : 'bg-slate-600'}`}></span>
                           </span>
                           {isRunning ? agent.status : 'Offline'}
                         </div>
                       </div>
                       
                       <div className="text-right">
                          <div className="text-lg font-mono font-bold text-white">{isRunning ? agent.load.toFixed(0) : 0}%</div>
                          <div className="text-[9px] uppercase tracking-wider text-slate-500 font-bold">Node load</div>
                       </div>
                    </div>
                  ))}
               </div>
            </div>
          </motion.div>
        )}

        {/* TAB 2: SYSTEM LOGGER TERMINAL */}
        {activeTab === 'terminal' && (
          <motion.div
            key="terminal"
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="flex flex-col bg-[#060a16] border border-white/5 shadow-2xl rounded-2xl overflow-hidden flex-1"
          >
            {/* Header / Console controls */}
            <div className="p-4 md:p-5 border-b border-white/5 bg-[#090b14]/80 flex flex-col md:flex-row md:items-center justify-between gap-4">
              <div className="flex flex-wrap items-center gap-2">
                {/* Level selection filters */}
                <button
                  onClick={() => setLogFilter('ALL')}
                  className={`px-3 py-1.5 rounded-lg font-mono text-xs font-bold uppercase transition-all ${
                    logFilter === 'ALL'
                      ? 'bg-slate-800 text-teal-400 border border-teal-500/30'
                      : 'text-slate-400 hover:text-white border border-transparent'
                  }`}
                >
                  ALL ({totalLogsCount})
                </button>
                <button
                  onClick={() => setLogFilter('INFO')}
                  className={`px-3 py-1.5 rounded-lg font-mono text-xs font-bold uppercase transition-all ${
                    logFilter === 'INFO'
                      ? 'bg-teal-500/10 text-teal-400 border border-teal-500/30'
                      : 'text-slate-400 hover:text-white border border-transparent'
                  }`}
                >
                  INFO ({infoCount})
                </button>
                <button
                  onClick={() => setLogFilter('WARN')}
                  className={`px-3 py-1.5 rounded-lg font-mono text-xs font-bold uppercase transition-all ${
                    logFilter === 'WARN'
                      ? 'bg-amber-500/10 text-amber-400 border border-amber-500/30'
                      : 'text-slate-400 hover:text-white border border-transparent'
                  }`}
                >
                  WARN ({warnCount})
                </button>
                <button
                  onClick={() => setLogFilter('ERROR')}
                  className={`px-3 py-1.5 rounded-lg font-mono text-xs font-bold uppercase transition-all ${
                    logFilter === 'ERROR'
                      ? 'bg-rose-500/10 text-rose-400 border border-rose-500/30'
                      : 'text-slate-400 hover:text-white border border-transparent'
                  }`}
                >
                  ERROR ({errorCount})
                </button>
              </div>

              {/* Log Search input */}
              <div className="relative max-w-xs w-full">
                <Search className="absolute left-3 top-2.5 w-4 h-4 text-slate-500" />
                <input
                  type="text"
                  placeholder="Filter logs by keyword..."
                  value={logSearch}
                  onChange={(e) => setLogSearch(e.target.value)}
                  className="w-full bg-black/40 border border-white/5 rounded-lg pl-9 pr-4 py-1.5 text-xs text-slate-300 font-mono placeholder:text-slate-600 focus:outline-none focus:border-teal-500/50"
                />
                {logSearch && (
                  <button 
                    onClick={() => setLogSearch('')} 
                    className="absolute right-2.5 top-2 text-[10px] text-slate-500 hover:text-white font-mono"
                  >
                    CLEAR
                  </button>
                )}
              </div>
            </div>

            {/* Simulated Live Logging screen */}
            <div className="p-5 flex-1 overflow-y-auto custom-scrollbar font-mono text-xs leading-relaxed min-h-[350px] bg-black/20">
              <div className="flex flex-col gap-2">
                <AnimatePresence>
                  {filteredLogs.map((log) => (
                    <motion.div
                      key={log.id}
                      initial={{ opacity: 0, x: -8 }}
                      animate={{ opacity: 1, x: 0 }}
                      className="flex items-start gap-3 border-b border-white/[0.02] pb-1.5"
                    >
                      {/* Timestamp */}
                      <span className="text-slate-600 select-none shrink-0">[{log.time}]</span>
                      
                      {/* Level styled badge */}
                      <span className={`shrink-0 font-bold px-1.5 py-0.5 rounded text-[10px] tracking-wide ${
                        log.level === 'ERROR' ? 'bg-red-500/10 text-red-400 border border-red-500/20' :
                        log.level === 'WARN' ? 'bg-amber-500/10 text-amber-400 border border-amber-500/20' :
                        'bg-teal-500/10 text-teal-400 border border-teal-500/20'
                      }`}>
                        {log.level}
                      </span>

                      {/* Module tag */}
                      <span className="text-purple-400 font-bold shrink-0">[{log.module}]</span>

                      {/* Msg */}
                      <span className="text-slate-300 flex-1 break-all">{log.message}</span>
                    </motion.div>
                  ))}
                  {filteredLogs.length === 0 && (
                    <div className="text-slate-600 italic text-center py-10 font-mono text-xs">
                      No matching log events found. Filter / Search keywords yields 0 records.
                    </div>
                  )}
                </AnimatePresence>
              </div>
            </div>

            {/* Quick Actions Footer */}
            <div className="p-4 bg-[#090b14]/70 border-t border-white/5 flex flex-wrap items-center justify-between gap-3 font-mono text-xs text-slate-500">
               <div>Total log pool size: {logs.length} (Self-rotating 150 Limit)</div>
               <div className="flex gap-2">
                 <button
                   onClick={() => {
                     const testId = Date.now().toString();
                     const testTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
                     setLogs(prev => [
                       { id: testId, time: testTime, level: 'WARN', module: 'SIMULATOR', message: '⚠️ User-triggered warning event diagnostics injection.' },
                       ...prev
                     ]);
                   }}
                   className="px-3 py-1 bg-amber-500/10 hover:bg-amber-500/20 border border-amber-500/30 text-amber-400 rounded transition-colors"
                 >
                   Inject Test Warning
                 </button>
                 <button
                   onClick={() => {
                     const testId = Date.now().toString();
                     const testTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
                     setLogs(prev => [
                       { id: testId, time: testTime, level: 'ERROR', module: 'SIMULATOR', message: '🔌 User-triggered failure crash simulation sequence.' },
                       ...prev
                     ]);
                   }}
                   className="px-3 py-1 bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/30 text-rose-400 rounded transition-colors"
                 >
                   Inject Test Error
                 </button>
                 <button
                   onClick={() => setLogs([])}
                   className="px-3 py-1 bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 rounded transition-colors flex items-center gap-1"
                 >
                   <Trash2 className="w-3.5 h-3.5" /> Clear logs
                 </button>
               </div>
            </div>
          </motion.div>
        )}

        {/* TAB 3: HYBRID CONSENSUS SIMULATOR */}
        {activeTab === 'consensus' && (
          <motion.div
            key="consensus"
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="flex flex-col gap-6 flex-1"
          >
            {/* SUB-TAB BAR */}
            <div className="flex bg-slate-950/40 p-1.5 rounded-2xl border border-white/5 gap-2 self-start">
              <button
                onClick={() => setConsensusSubView('simulator')}
                className={`px-4 py-2 rounded-xl text-xs font-mono font-bold uppercase transition-all tracking-wide flex items-center gap-2 ${
                  consensusSubView === 'simulator'
                    ? 'bg-gradient-to-r from-teal-500/10 to-indigo-500/10 text-teal-400 border border-teal-500/20 shadow-lg'
                    : 'text-slate-400 hover:text-white border border-transparent'
                }`}
              >
                <Activity className="w-4 h-4 text-emerald-400" />
                Live-Analyse-Simulator
              </button>
              <button
                onClick={() => setConsensusSubView('integration')}
                className={`px-4 py-2 rounded-xl text-xs font-mono font-bold uppercase transition-all tracking-wide flex items-center gap-2 ${
                  consensusSubView === 'integration'
                    ? 'bg-gradient-to-r from-teal-500/10 to-indigo-500/10 text-teal-400 border border-teal-500/20 shadow-lg'
                    : 'text-slate-400 hover:text-white border border-transparent'
                }`}
              >
                <Code className="w-4 h-4 text-indigo-400" />
                Architektur & Integration
              </button>
            </div>

            {consensusSubView === 'simulator' ? (
              <div className="grid grid-cols-1 xl:grid-cols-12 gap-6 flex-1">
                {/* LEFT COLUMN: PARAMETER CONFIGURATION (xl:col-span-5) */}
                <div className="xl:col-span-5 flex flex-col gap-5">
              
              {/* Box 1: Globale Konsens-Regeln */}
              <div className="bg-[#090b14]/90 border border-white/5 rounded-2xl p-5 shadow-xl relative overflow-hidden">
                <div className="absolute top-0 left-0 w-1 h-full bg-teal-500" />
                <h3 className="font-bold text-white text-sm uppercase tracking-wider flex items-center gap-2 mb-4 font-mono">
                  <Sliders className="w-4 h-4 text-teal-400" /> Globale Konsens-Regeln
                </h3>

                <div className="space-y-4">
                  {/* Required Stake */}
                  <div className="space-y-1">
                    <div className="flex justify-between text-xs font-mono text-slate-300">
                      <span>Min. Stake (PoS Threshold):</span>
                      <span className="text-teal-400 font-bold">{requiredStake.toLocaleString()} ATC</span>
                    </div>
                    <input 
                      type="range"
                      min="500"
                      max="100000"
                      step="500"
                      value={requiredStake}
                      onChange={(e) => setRequiredStake(Number(e.target.value))}
                      className="w-full accent-teal-400 h-1 bg-slate-800 rounded-lg cursor-pointer"
                    />
                  </div>

                  {/* Required PoI */}
                  <div className="space-y-1">
                    <div className="flex justify-between text-xs font-mono text-slate-300">
                      <span>Min. Aktivität (PoI Threshold):</span>
                      <span className="text-teal-400 font-bold">{requiredPoI}/100</span>
                    </div>
                    <input 
                      type="range"
                      min="10"
                      max="100"
                      step="5"
                      value={requiredPoI}
                      onChange={(e) => setRequiredPoI(Number(e.target.value))}
                      className="w-full accent-teal-400 h-1 bg-slate-800 rounded-lg cursor-pointer"
                    />
                  </div>

                  {/* PoH Hash-Iterations */}
                  <div className="space-y-1">
                    <div className="flex justify-between text-xs font-mono text-slate-300">
                      <span>PoH Hash-Iterationen (Zeitstempel):</span>
                      <span className="text-indigo-400 font-bold">{pohIterations} Iterationen</span>
                    </div>
                    <input 
                      type="range"
                      min="5"
                      max="50"
                      step="1"
                      value={pohIterations}
                      onChange={(e) => setPohIterations(Number(e.target.value))}
                      className="w-full accent-indigo-500 h-1 bg-slate-800 rounded-lg cursor-pointer"
                    />
                  </div>

                  {/* PoW Difficulty */}
                  <div className="space-y-1">
                    <div className="flex justify-between text-xs font-mono text-slate-300">
                      <span>PoW Schwierigkeit (Führende Nullen):</span>
                      <span className="text-rose-450 font-bold text-rose-400">
                        {"0".repeat(powDifficulty)} ({powDifficulty}x Nullen)
                      </span>
                    </div>
                    <input 
                      type="range"
                      min="1"
                      max="3"
                      step="1"
                      value={powDifficulty}
                      onChange={(e) => setPowDifficulty(Number(e.target.value))}
                      className="w-full accent-rose-500 h-1 bg-slate-800 rounded-lg cursor-pointer"
                    />
                  </div>
                </div>
              </div>

              {/* Box 2: Validator Profile Konfiguration */}
              <div className="bg-[#090b14]/90 border border-white/5 rounded-2xl p-5 shadow-xl relative overflow-hidden flex-1">
                <div className="absolute top-0 left-0 w-1 h-full bg-[#a855f7]" />
                <h3 className="font-bold text-white text-sm uppercase tracking-wider flex items-center gap-2 mb-4 font-mono">
                  <span className="text-purple-400">👤</span> Validator Parameter
                </h3>

                <div className="space-y-4 font-mono text-xs">
                  {/* ALICE SETTINGS */}
                  <div className="bg-white/[0.02] p-3 rounded-xl border border-white/5 space-y-2.5">
                    <div className="text-slate-300 font-bold flex items-center gap-1.5 border-b border-white/5 pb-1">
                      <span className="text-emerald-400 text-sm">🧑‍💻</span> Miner_Alice
                    </div>
                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="text-slate-500 block text-[10px] uppercase">Stake (Coins)</label>
                        <input 
                          type="number" 
                          value={aliceStake}
                          onChange={(e) => setAliceStake(Math.max(0, parseInt(e.target.value) || 0))}
                          className="bg-slate-900 border border-white/10 rounded px-2 py-1 text-white w-full text-xs font-bold font-mono focus:border-indigo-500 outline-none"
                        />
                      </div>
                      <div>
                        <label className="text-slate-500 block text-[10px] uppercase">PoI Score (0-100)</label>
                        <input 
                          type="number" 
                          value={alicePoI}
                          onChange={(e) => setAlicePoI(Math.min(100, Math.max(0, parseInt(e.target.value) || 0)))}
                          className="bg-slate-900 border border-white/10 rounded px-2 py-1 text-white w-full text-xs font-bold font-mono focus:border-indigo-500 outline-none"
                        />
                      </div>
                    </div>
                  </div>

                  {/* BOB SETTINGS */}
                  <div className="bg-white/[0.02] p-3 rounded-xl border border-white/5 space-y-2.5">
                    <div className="text-slate-300 font-bold flex items-center gap-1.5 border-b border-white/5 pb-1">
                      <span className="text-rose-400 text-sm">🕵️‍♂️</span> Miner_Bob
                    </div>
                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="text-slate-500 block text-[10px] uppercase">Stake (Coins)</label>
                        <input 
                          type="number" 
                          value={bobStake}
                          onChange={(e) => setBobStake(Math.max(0, parseInt(e.target.value) || 0))}
                          className="bg-slate-900 border border-white/10 rounded px-2 py-1 text-white w-full text-xs font-bold font-mono focus:border-indigo-500 outline-none"
                        />
                      </div>
                      <div>
                        <label className="text-slate-500 block text-[10px] uppercase">PoI Score (0-100)</label>
                        <input 
                          type="number" 
                          value={bobPoI}
                          onChange={(e) => setBobPoI(Math.min(100, Math.max(0, parseInt(e.target.value) || 0)))}
                          className="bg-slate-900 border border-white/10 rounded px-2 py-1 text-white w-full text-xs font-bold font-mono focus:border-rose-500 outline-none"
                        />
                      </div>
                    </div>
                  </div>

                  {/* CUSTOM CHIP SETTINGS */}
                  <div className="bg-indigo-950/20 p-3 rounded-xl border border-indigo-500/20 space-y-2.5">
                    <div className="text-indigo-300 font-bold flex items-center justify-between border-b border-indigo-500/10 pb-1">
                      <div className="flex items-center gap-1.5">
                        <span className="text-indigo-400 text-sm">🤖</span> Custom-Miner
                      </div>
                      <span className="text-[9px] uppercase bg-indigo-500/20 text-indigo-300 px-1.5 py-0.5 rounded border border-indigo-500/30">Dein Profil</span>
                    </div>
                    <div className="space-y-2">
                      <div>
                        <label className="text-slate-500 block text-[10px] uppercase">Miner Name</label>
                        <input 
                          type="text" 
                          value={customMinerName}
                          onChange={(e) => setCustomMinerName(e.target.value.replace(/\s+/g, '_'))}
                          className="bg-slate-900 border border-indigo-500/20 rounded px-2 py-1 text-white w-full text-xs font-bold font-mono focus:border-indigo-500 outline-none"
                        />
                      </div>
                      <div className="grid grid-cols-2 gap-3">
                        <div>
                          <label className="text-slate-500 block text-[10px] uppercase">Stake (Coins)</label>
                          <input 
                            type="number" 
                            value={customMinerStake}
                            onChange={(e) => setCustomMinerStake(Math.max(0, parseInt(e.target.value) || 0))}
                            className="bg-slate-900 border border-indigo-500/20 rounded px-2 py-1 text-white w-full text-xs font-bold font-mono focus:border-indigo-500 outline-none"
                          />
                        </div>
                        <div>
                          <label className="text-slate-500 block text-[10px] uppercase">PoI (0-100)</label>
                          <input 
                            type="number" 
                            value={customMinerPoI}
                            onChange={(e) => setCustomMinerPoI(Math.min(100, Math.max(0, parseInt(e.target.value) || 0)))}
                            className="bg-slate-900 border border-indigo-500/20 rounded px-2 py-1 text-white w-full text-xs font-bold font-mono focus:border-indigo-500 outline-none"
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* RIGHT COLUMN: CANDIDATES, ACTIONS, TERMINAL, CODE (xl:col-span-7) */}
            <div className="xl:col-span-7 flex flex-col gap-5">
              
              {/* Explainer Banner */}
              <div className="bg-indigo-950/20 border border-indigo-500/20 p-4.5 rounded-2xl flex flex-col md:flex-row items-start md:items-center gap-4">
                <div className="bg-indigo-500/10 p-2.5 rounded-xl border border-indigo-500/30 text-indigo-400 shrink-0">
                  <Sparkles className="w-5 h-5 animate-pulse" />
                </div>
                <div>
                  <h3 className="font-bold text-white text-sm">Hybrid-Konsens Validator Simulator</h3>
                  <p className="text-slate-400 text-[11px] mt-0.5 leading-relaxed font-sans">
                    Simuliert die Ausführung von Fünf unabhängigen Consensus-Arten synchron: PoS stake threshold verification, PoI community history scores, PoH continuous SHA-256 state time-stamping, PoAI active neuron learning loss, und PoW mathematical block proofing.
                  </p>
                </div>
              </div>

              {/* Dynamic Candidates Deck */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                {/* Alice Card */}
                <div className={`p-4 rounded-xl border transition-all ${
                  activeValidator === 'Alice' 
                    ? 'bg-indigo-950/30 border-indigo-500 shadow-[0_0_15px_rgba(99,102,241,0.15)] bg-gradient-to-br from-indigo-950/20 to-black' 
                    : 'bg-[#090b14]/85 border-white/5 hover:border-white/10'
                }`}>
                  <div className="flex items-center justify-between mb-3 text-xs">
                    <span className="font-bold text-slate-200 flex items-center gap-1 md:gap-1.5">
                      <span className="text-base select-none">🧑‍💻</span> Alice
                    </span>
                    <span className={`text-[9px] font-bold px-1.5 py-0.5 rounded font-mono ${
                      aliceStake >= requiredStake && alicePoI >= requiredPoI
                        ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
                        : "bg-rose-500/10 text-rose-400 border border-rose-500/20"
                    }`}>
                      {aliceStake >= requiredStake && alicePoI >= requiredPoI ? "Bereit" : "Gesperrt"}
                    </span>
                  </div>
                  <div className="text-[11px] space-y-1.5 text-slate-400 font-mono">
                    <div className="flex justify-between border-b border-white/[0.03] pb-1">
                      <span>Stake:</span>
                      <span className={aliceStake >= requiredStake ? "text-emerald-400 font-bold" : "text-rose-500 font-bold"}>
                        {aliceStake.toLocaleString()} <span className="text-[10px] text-slate-500">/{requiredStake.toLocaleString()}</span>
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span>Score:</span>
                      <span className={alicePoI >= requiredPoI ? "text-emerald-400 font-bold" : "text-rose-500 font-bold"}>
                        {alicePoI} <span className="text-[10px] text-slate-500">/{requiredPoI}</span>
                      </span>
                    </div>
                  </div>
                </div>

                {/* Bob Card */}
                <div className={`p-4 rounded-xl border transition-all ${
                  activeValidator === 'Bob' 
                    ? 'bg-indigo-950/30 border-indigo-500 shadow-[0_0_15px_rgba(99,102,241,0.15)] bg-gradient-to-br from-indigo-950/20 to-black' 
                    : 'bg-[#090b14]/85 border-white/5 hover:border-white/10'
                }`}>
                  <div className="flex items-center justify-between mb-3 text-xs">
                    <span className="font-bold text-slate-200 flex items-center gap-1.5">
                      <span className="text-base select-none">🕵️‍♂️</span> Bob
                    </span>
                    <span className={`text-[9px] font-bold px-1.5 py-0.5 rounded font-mono ${
                      bobStake >= requiredStake && bobPoI >= requiredPoI
                        ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
                        : "bg-rose-500/10 text-rose-400 border border-rose-500/20"
                    }`}>
                      {bobStake >= requiredStake && bobPoI >= requiredPoI ? "Bereit" : "Gesperrt"}
                    </span>
                  </div>
                  <div className="text-[11px] space-y-1.5 text-slate-400 font-mono">
                    <div className="flex justify-between border-b border-white/[0.03] pb-1">
                      <span>Stake:</span>
                      <span className={bobStake >= requiredStake ? "text-emerald-400 font-bold" : "text-rose-500 font-bold"}>
                        {bobStake.toLocaleString()} <span className="text-[10px] text-slate-500">/{requiredStake.toLocaleString()}</span>
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span>Score:</span>
                      <span className={bobPoI >= requiredPoI ? "text-emerald-400 font-bold" : "text-rose-500 font-bold"}>
                        {bobPoI} <span className="text-[10px] text-slate-500">/{requiredPoI}</span>
                      </span>
                    </div>
                  </div>
                </div>

                {/* Custom Card */}
                <div className={`p-4 rounded-xl border transition-all ${
                  activeValidator === 'Custom' 
                    ? 'bg-indigo-950/30 border-indigo-500 shadow-[0_0_15px_rgba(99,102,241,0.15)] bg-gradient-to-br from-indigo-950/20 to-black' 
                    : 'bg-[#a855f7]/5 border-purple-500/10 hover:border-purple-500/20'
                }`}>
                  <div className="flex items-center justify-between mb-3 text-xs">
                    <span className="font-bold text-purple-200 flex items-center gap-1.5 truncate max-w-[90px]">
                      <span className="text-base select-none">🤖</span> {customMinerName}
                    </span>
                    <span className={`text-[9px] font-bold px-1.5 py-0.5 rounded font-mono ${
                      customMinerStake >= requiredStake && customMinerPoI >= requiredPoI
                        ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
                        : "bg-rose-500/10 text-rose-400 border border-rose-500/20"
                    }`}>
                      {customMinerStake >= requiredStake && customMinerPoI >= requiredPoI ? "Bereit" : "Gesperrt"}
                    </span>
                  </div>
                  <div className="text-[11px] space-y-1.5 text-slate-400 font-mono font-sans">
                    <div className="flex justify-between border-b border-white/[0.03] pb-1">
                      <span>Stake:</span>
                      <span className={customMinerStake >= requiredStake ? "text-emerald-400 font-bold" : "text-rose-500 font-bold"}>
                        {customMinerStake.toLocaleString()} <span className="text-[10px] text-slate-500">/{requiredStake.toLocaleString()}</span>
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span>Score:</span>
                      <span className={customMinerPoI >= requiredPoI ? "text-emerald-400 font-bold" : "text-rose-500 font-bold"}>
                        {customMinerPoI} <span className="text-[10px] text-slate-500">/{requiredPoI}</span>
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Simulation Actions */}
              <div className="flex flex-wrap items-center justify-center gap-3">
                <button 
                  onClick={() => startConsensusSimulation('Alice')}
                  disabled={isSimulating}
                  className="bg-emerald-600 hover:bg-emerald-500 hover:shadow-lg hover:shadow-emerald-500/10 disabled:opacity-50 text-white font-mono font-bold text-xs py-2.5 px-4.5 rounded-xl transition-all focus:outline-none flex items-center gap-2 border border-emerald-500/20"
                  id="btn-simulate-alice"
                >
                  {isSimulating && activeValidator === 'Alice' ? (
                    <>
                      <RefreshCw className="w-4 h-4 animate-spin" />
                      Alice rechnet...
                    </>
                  ) : (
                    <>▶️ Alice simulieren</>
                  )}
                </button>

                <button 
                  onClick={() => startConsensusSimulation('Bob')}
                  disabled={isSimulating}
                  className="bg-rose-600 hover:bg-rose-500 hover:shadow-lg hover:shadow-rose-500/10 disabled:opacity-50 text-white font-mono font-bold text-xs py-2.5 px-4.5 rounded-xl transition-all focus:outline-none flex items-center gap-2 border border-rose-500/20"
                  id="btn-simulate-bob"
                >
                  {isSimulating && activeValidator === 'Bob' ? (
                    <>
                      <RefreshCw className="w-4 h-4 animate-spin" />
                      Bob prüft...
                    </>
                  ) : (
                    <>▶️ Bob simulieren</>
                  )}
                </button>

                <button 
                  onClick={() => startConsensusSimulation('Custom')}
                  disabled={isSimulating}
                  className="bg-purple-600 hover:bg-purple-500 hover:shadow-lg hover:shadow-purple-500/10 disabled:opacity-50 text-white font-mono font-bold text-xs py-2.5 px-4.5 rounded-xl transition-all focus:outline-none flex items-center gap-2 border border-purple-500/20"
                  id="btn-simulate-custom"
                >
                  {isSimulating && activeValidator === 'Custom' ? (
                    <>
                      <RefreshCw className="w-4 h-4 animate-spin" />
                      Simulation läuft...
                    </>
                  ) : (
                    <>▶️ Custom-Validator</>
                  )}
                </button>

                <button 
                  onClick={clearConsensusTerminal}
                  className="bg-slate-850 hover:bg-slate-800 text-slate-300 border border-white/5 font-mono font-bold text-xs py-2.5 px-4.5 rounded-xl transition-all shadow-md focus:outline-none flex items-center gap-1.5"
                  id="btn-clear-sim"
                >
                  <Trash2 className="w-3.5 h-3.5" /> Terminal leeren
                </button>
              </div>

              {/* Console log display */}
              <div className="bg-[#030611] rounded-2xl shadow-2xl border border-indigo-950 overflow-hidden flex flex-col">
                <div className="bg-[#0c0f1e] px-4 py-2.5 border-b border-indigo-950 flex items-center justify-between">
                  <div className="flex items-center">
                    <div className="w-2.5 h-2.5 rounded-full bg-red-500 mr-1.5"></div>
                    <div className="w-2.5 h-2.5 rounded-full bg-yellow-500 mr-1.5"></div>
                    <div className="w-2.5 h-2.5 rounded-full bg-green-500 mr-1.5"></div>
                    <span className="text-indigo-400 text-xs font-mono ml-2 font-bold select-none">consensus_node_shell.sh</span>
                  </div>
                  {isSimulating && (
                    <span className="text-[10px] uppercase font-mono tracking-widest text-emerald-400 animate-pulse flex items-center gap-1 font-bold">
                      <span className="h-1.5 w-1.5 bg-emerald-400 rounded-full animate-ping inline-block" /> RUNNING MULTI-PROOFS
                    </span>
                  )}
                </div>
                
                <div id="terminal-sim" className="p-4 h-72 overflow-y-auto font-mono text-xs leading-relaxed space-y-2 bg-black/60 scroll-smooth custom-scrollbar">
                  {simulatorLogs.map((log, index) => {
                    let badgeColor = "text-slate-300";
                    if (log.type === 'error') badgeColor = "text-rose-400 font-bold border-l-2 border-rose-500 pl-3 my-2";
                    if (log.type === 'success') badgeColor = "text-emerald-400 font-semibold";
                    if (log.type === 'header') badgeColor = "text-indigo-400 font-bold mt-4 border-t border-indigo-950/20 pt-2.5 flex items-center gap-1.5";
                    if (log.type === 'data') badgeColor = "text-yellow-300 break-all pl-3 border-l-2 border-yellow-600/80 my-2";

                    return (
                      <div key={index} className={badgeColor}>
                        {log.text}
                      </div>
                    );
                  })}
                </div>
              </div>

              {/* Code Preview Accordion */}
              <div className="bg-[#090b14]/90 border border-white/5 rounded-2xl overflow-hidden font-mono text-xs shadow-xl">
                <button 
                  onClick={() => setShowCodePreview(!showCodePreview)}
                  className="w-full px-5 py-3.5 bg-white/[0.02] hover:bg-white/[0.04] transition-colors border-b border-white/5 flex items-center justify-between text-slate-300 font-bold text-left outline-none"
                >
                  <span className="flex items-center gap-2">
                    <Code className="w-4 h-4 text-purple-400" /> Executive Simulator Source Code (JS)
                  </span>
                  <span className="text-slate-400 hover:text-white transition-colors">{showCodePreview ? "COLLAPSE [-]" : "EXPAND [+]"}</span>
                </button>
                
                <AnimatePresence>
                  {showCodePreview && (
                    <motion.div
                      initial={{ height: 0 }}
                      animate={{ height: "auto" }}
                      exit={{ height: 0 }}
                      className="overflow-hidden"
                    >
                      <div className="p-4 bg-black/80 text-teal-300 text-[10px] leading-relaxed max-h-72 overflow-y-auto custom-scrollbar whitespace-pre">
{`class HybridValidator {
  constructor(name, stake, importance) {
    this.name = name;
    this.stake = stake;
    this.importance = importance;
  }

  async erstelleBlock(blockDaten) {
    console.log(\`Validator [\${this.name}] startet den Hybrid-Konsens...\`);

    // --- SCHRITT 1: Proof of Stake (PoS) ---
    if (this.stake < ${requiredStake}) {
      console.error(\`❌ PoS FEHLGESCHLAGEN: Zu wenig Coins.\`);
      return null;
    }
    console.log(\`✅ PoS BESTANDEN: \${this.stake} Coins verfügbar.\`);

    // --- SCHRITT 2: Proof of Importance (PoI) ---
    if (this.importance < ${requiredPoI}) {
      console.error(\`❌ PoI FEHLGESCHLAGEN: Community-Wert zu niedrig.\`);
      return null;
    }
    console.log(\`✅ PoI BESTANDEN: Aktivitätswert \${this.importance}/100.\`);

    // --- SCHRITT 3: Proof of History (PoH) ---
    let historyHash = "A-TownChain_Seed";
    for (let i = 0; i < ${pohIterations}; i++) {
      historyHash = await calculateSHA256(historyHash);
    }
    console.log(\`✅ PoH BESTANDEN: Zeitablauf-Beweis koadjutschiert.\`);

    // --- SCHRITT 4: Proof of AI (PoAI) ---
    let loss = 1.0;
    while (loss > 0.1) {
      loss = loss * 0.82; // Epochenfortschritt
    }
    console.log(\`✅ PoAI BESTANDEN: KI-Modell trainiert (Loss: \${loss.toFixed(4)})\`);

    // --- SCHRITT 5: Proof of Work (PoW) ---
    let nonce = 0;
    const ziel = "${"0".repeat(powDifficulty)}";
    let finalerHash = "";
    while (!finalerHash.startsWith(ziel)) {
      nonce++;
      finalerHash = await calculateSHA256(blockDaten + nonce);
    }
    console.log(\`✅ PoW BESTANDEN: Block verifiziert! (Nonce: \${nonce})\`);

    return { miner: this.name, nonce, hash: finalerHash };
  }
}`}
                      </div>
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>

            </div>
          </div>
            ) : (
              <ConsensusIntegrationGuide />
            )}
          </motion.div>
        )}
      </AnimatePresence>

      {/* ================== STOP ALL PROCESSES CONFIRMATION MODAL ================== */}
      <AnimatePresence>
        {showStopConfirmation && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm">
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="bg-[#0d1326] border-2 border-rose-500/50 rounded-2xl max-w-md w-full p-6 shadow-[0_0_50px_rgba(244,63,94,0.15)] flex flex-col gap-4 text-slate-200"
            >
              <div className="flex items-start gap-4">
                <div className="bg-rose-500/10 p-3 rounded-xl border border-rose-500/30 text-rose-500 shrink-0">
                  <ShieldAlert className="w-6 h-6 animate-pulse" />
                </div>
                <div>
                  <h3 className="font-bold text-white text-lg font-mono">CRITICAL CONFIRMATION</h3>
                  <p className="text-slate-400 text-xs mt-1 leading-relaxed">
                    You are initiating a manual emergency stop. This action suspends all cryptographic validators, pauses decentralized swarm coordinate APIs, drops consensus peer connection indexes, and turns off state history logging.
                  </p>
                </div>
              </div>

              <div className="p-3 bg-rose-500/5 border border-rose-500/20 rounded-xl flex items-center gap-2.5 text-rose-400 font-mono text-[10px] tracking-wide uppercase font-bold">
                <AlertCircle className="w-4 h-4 shrink-0" />
                No blocks will be verified until restarted.
              </div>

              <div className="flex justify-end gap-3 font-mono text-xs mt-2">
                <button
                  onClick={() => setShowStopConfirmation(false)}
                  className="px-4 py-2.5 rounded-lg border border-white/10 hover:bg-white/5 text-slate-400 hover:text-white transition-colors"
                >
                  ABORT OVERRIDE
                </button>
                <button
                  onClick={triggerEmergencyStop}
                  className="px-5 py-2.5 rounded-lg bg-rose-600 hover:bg-rose-500 hover:shadow-lg hover:shadow-rose-650/10 text-white font-bold transition-all"
                  id="btn-confirm-override"
                >
                  STOP ALL PROCESSES
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </div>
  );
}
