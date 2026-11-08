import React, { useState, useEffect } from 'react';
import { Layers, Zap, Power, PowerOff, Cpu, Box, FileText, Code2, GitMerge } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { TooltipIcon } from './TooltipIcon';

type PluginModule = {
  id: string;
  name: string;
  category: string;
  loadAvg: number;
  isActive: boolean;
  activeSince?: number;
};

const MOCK_MODULES: PluginModule[] = [
  { id: 'mod-1', name: 'ZkStateProvider', category: 'Security', loadAvg: 0, isActive: false },
  { id: 'mod-2', name: 'SocketStreamer', category: 'Network', loadAvg: 12.4, isActive: true, activeSince: Date.now() - 34000 },
  { id: 'mod-3', name: 'MemoryGC', category: 'System', loadAvg: 0, isActive: false },
  { id: 'mod-4', name: 'P2P_Gossip', category: 'Network', loadAvg: 45.2, isActive: true, activeSince: Date.now() - 120000 },
  { id: 'mod-5', name: 'HashTreeBuilder', category: 'Core', loadAvg: 0, isActive: false },
  { id: 'mod-6', name: 'DataLakeSink', category: 'Data', loadAvg: 8.1, isActive: true, activeSince: Date.now() - 5000 },
  { id: 'mod-7', name: 'AuthEnclave', category: 'Security', loadAvg: 0, isActive: false },
];

const SPEC_CONCEPTS = [
  {
    id: "vars",
    name: "1. Variablen & Konstanten",
    desc: "Syntax für veränderbare/unveränderbare Werte und integrierte Typisierungen.",
    code: `// standardmäßig unveränderbar (Konstanten) mit "let"
let block_height: u64 = 142000;
let dev_wallet: Address = "0xATC_DEVELOPER_POOL";
let is_secure: bool = true;

// veränderbare Variablen mit dem expliziten "mut" Modifikator (mutable)
let mut current_tps: u64 = 45000;
current_tps = 48500; // Gültige Neuzuweisung

// Typisierungen in atc-lang:
// - u32 / u64 / u256 (Vorzeichenlose Ganzzahlen, u256 ideal für native Assets)
// - f32 / f64 (Gleitkommazahlen, z.B. für AI Model Loss-Werte)
// - bool (Wahrheitswert: true, false)
// - String (Unicode Zeichenketten)
// - Address (Kryptografische Wallet- oder Contract-Adressen)
// - Hash256 (Kompakter 256-Bit SHA-Auswertungs-Hash)`
  },
  {
    id: "funcs",
    name: "2. Funktionen (Routinen)",
    desc: "Wie man deterministische Routinen mit Parametern und Typ-Rückgaben deklariert.",
    code: `// Routinen werden mit dem Schlüsselwort "routine" deklariert.
// Verwenden Sie "pub" (public) oder "priv" (private) für die Zugriffskontrolle.
pub routine addiere_stake(miner: Address, amount: u256) -> u256 {
    let current_stake: u256 = System.get_stake(miner);
    let updated_stake: u256 = current_stake + amount;
    return updated_stake;
}

// Routine ohne expliziten Rückgabewert (void)
pub routine alert_validator(id: u64) {
    System.log("Validator-Warnung ausgelöst: ID " + id);
}`
  },
  {
    id: "control",
    name: "3. Kontrollstrukturen",
    desc: "Bedingungen (If/Else) und ressourcen-beschränkte Schleifen/Wegekontrollen.",
    code: `// 1. Bedingungsverzweigung (If / Else If / Else)
let miner_stake: u256 = System.get_stake(miner_addr);
if miner_stake >= 10000 {
    System.log("Stake verifiziert.");
} else if miner_stake > 5000 {
    System.log("Teilweiser Stake vorhanden. Warnung.");
} else {
    System.panic("Stake ungenügend! Validierung abgebrochen.");
}

// 2. Bereichs-basierte Schleifen (loop i from [x] to [y]) - Extrem gas-sicher!
let mut running_hash: Hash256 = block_prev_hash;
loop i from 0 to 5000 {
    running_hash = sha256(running_hash); // Proof of History Simulation
}

// 3. Flexibler While-Loop für allgemeine Logik
let mut retries: u64 = 3;
while retries > 0 {
    retries = retries - 1;
}`
  },
  {
    id: "data",
    name: "4. Datenstrukturen",
    desc: "Listen-Typen und Key-Value-Speicher (Helper/Collections) in atc-lang.",
    code: `// 1. Arrays/Listen (List-Typ) befüllen und abrufen
let mut transaction_queue: List<Hash256> = init List();
transaction_queue.push("0xHashA...");
transaction_queue.push("0xHashB...");

let first_tx: Hash256 = transaction_queue.get(0);

// 2. Tabellen/Mapparieren (Map-Typ) für persistenten Heap-Zustand
let mut score_table: Map<Address, u8> = init Map();
score_table.set("0xMinerAddressA", 85);

// Abrufen mit Null-Coalescing-Fallback ("?:") für maximale Sicherheit
let final_score: u8 = score_table.get("0xMinerAddressA") ?: 0;`
  },
  {
    id: "bitwise",
    name: "5. Bitweise Operatoren",
    desc: "Low-Level-Manipulation von Registern - Essentiell für SHA-256 Berechnungen.",
    code: `// Bitweise Basis-Operatoren auf Ganzzahlen (u32/u64/u256)
let word_a: u32 = 0x5a827999;
let word_b: u32 = 0x6ed9eba1;

let op_and: u32 = word_a & word_b;  // Bitweises AND
let op_or: u32 = word_a | word_b;   // Bitweises OR
let op_xor: u32 = word_a ^ word_b;  // Bitweises XOR

// Bit-Shifting (Verschiebungen)
let shift_left: u32 = word_a << 8;    // Shift Left (Mal 256)
let shift_right: u32 = word_b >> 4;   // Logical Shift Right

// Native Rotation (hochoptimierter Direct-ASM-Ersatz für Krypto-Routinen)
let rotated_l: u32 = rotate_left(word_a, 5);
let rotated_r: u32 = rotate_right(word_b, 17);`
  },
  {
    id: "structs",
    name: "6. Objektorientierung / Structs",
    desc: "Definition strukturierter Blockchain-Körper und Validator-Daten.",
    code: `// Definition des Datentyps für Verifizierung und Transport
struct BlockCompact {
    height: u64
    prev_hash: Hash256
    miner: Address
    timestamp: u64
    nonce: u64
    poai_loss: f32
}

// Struct Erstellung mit dem "init" Keyword
let local_block = init BlockCompact {
    height: 1251,
    prev_hash: "0x0A9F...",
    miner: "0xMiner1",
    timestamp: 1774894101,
    nonce: 871239,
    poai_loss: 0.042
};

// Direkter punktbasierter Member-Zugriff
let current_loss: f32 = local_block.poai_loss;`
  }
];

export function AtcLangArchitectureView() {
  const [modules, setModules] = useState<PluginModule[]>(MOCK_MODULES);
  const [activeTab, setActiveTab] = useState<'kernel' | 'compiler' | 'syntax'>('compiler');
  const [selectedSpec, setSelectedSpec] = useState<string>("vars");

  // Simulate dynamic loading/unloading
  useEffect(() => {
    const interval = setInterval(() => {
      if (activeTab !== 'kernel') return;
      setModules(prev => prev.map(mod => {
        // Random chance to toggle state
        if (Math.random() > 0.8) {
          const activating = !mod.isActive;
          return {
            ...mod,
            isActive: activating,
            loadAvg: activating ? Math.random() * 80 + 5 : 0,
            activeSince: activating ? Date.now() : undefined
          };
        }
        
        // Randomly adjust load metrics
        if (mod.isActive) {
          return {
            ...mod,
            loadAvg: Math.max(0.1, Math.min(100, mod.loadAvg + (Math.random() * 20 - 10)))
          };
        }
        
        return mod;
      }));
    }, 2500);
    
    return () => clearInterval(interval);
  }, [activeTab]);

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="flex gap-4 p-4 border-b border-white/5 bg-black/20 overflow-x-auto custom-scrollbar shrink-0">
        <button
          onClick={() => setActiveTab('compiler')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors shrink-0 ${
            activeTab === 'compiler'
              ? 'bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/30 shadow-[0_0_15px_rgba(6,182,212,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <GitMerge className="w-4 h-4" />
          Compiler Pipeline
        </button>
        <button
          onClick={() => setActiveTab('syntax')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors shrink-0 ${
            activeTab === 'syntax'
              ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 shadow-[0_0_15px_rgba(16,185,129,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Code2 className="w-4 h-4" />
          Custom Syntax Design
        </button>
        <button
          onClick={() => setActiveTab('kernel')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors shrink-0 ${
            activeTab === 'kernel'
              ? 'bg-atc-purple/20 text-atc-purple border border-atc-purple/30 shadow-[0_0_15px_rgba(162,89,255,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Layers className="w-4 h-4" />
          Core Kernel Plugins
        </button>
      </div>

      <div className="flex-1 overflow-auto p-6 font-sans w-full max-w-6xl mx-auto">
        <div className="mb-8 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-2xl bg-white/5 border border-white/10 flex items-center justify-center text-white shadow-lg">
              {activeTab === 'compiler' && <GitMerge className="w-6 h-6 text-atc-cyan" />}
              {activeTab === 'syntax' && <Code2 className="w-6 h-6 text-emerald-400" />}
              {activeTab === 'kernel' && <Layers className="w-6 h-6 text-atc-purple" />}
            </div>
            <div>
              <h2 className="text-3xl font-semibold tracking-tight text-white mb-1">
                {activeTab === 'compiler' && 'ATC-Lang Compiler Pipeline'}
                {activeTab === 'syntax' && 'Eigene ATC-Syntax & Lexer'}
                {activeTab === 'kernel' && 'ATVM Core Module Load'}
              </h2>
              <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
                {activeTab === 'compiler' && 'Eine komplett selbstentwickelte Architektur: Vom Text-Stream über den eigenen Tokenizer (Lexer), Abstract Syntax Tree (Parser) hin zur Semantischen Analyse und ATVM-Bytecode Emit. Alles 100% nativ in atc-lang ohne externe Abhängigkeiten.'}
                {activeTab === 'syntax' && 'ATC-Lang operiert mit starker, unumstößlicher Typisierung und eigenen Sprachkonstrukten für Blockchain-native Operationen, Zero-Knowledge Proofs und dezentrale Storage-Referenzen.'}
                {activeTab === 'kernel' && 'Dynamisches Core-Modulsystem: Module werden bei Bedarf nahtlos als eigene isolierte Layer in die Virtual Machine geladen.'}
              </p>
            </div>
          </div>
        </div>

        {activeTab === 'kernel' && (
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
            <div className="col-span-1 lg:col-span-8 flex flex-col gap-6">
              <div className="p-6 rounded-2xl bg-[#090b14]/80 backdrop-blur border border-white/5 shadow-xl relative overflow-hidden">
                <div className="absolute top-0 right-0 w-32 h-32 bg-atc-purple/5 rounded-full blur-[50px] pointer-events-none" />
                <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2 relative z-10">
                  <Box className="w-5 h-5 text-atc-purple" /> Plugin Layer Matrix 
                  <TooltipIcon content="Aktive Module werden hochgefahren und beanspruchen CPU. Inaktive Module verbleiben im Cold Storage." />
                </h3>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 relative z-10">
                  <AnimatePresence>
                    {modules.map(mod => (
                      <motion.div 
                        layout
                        key={mod.id} 
                        className={`p-4 rounded-xl border flex flex-col gap-3 transition-colors ${mod.isActive ? 'bg-black/60 border-atc-purple/30' : 'bg-black/20 border-white/5 opacity-60'}`}
                      >
                        <div className="flex items-start justify-between">
                          <div className="flex items-center gap-2">
                            {mod.isActive ? <Power className="w-4 h-4 text-emerald-400" /> : <PowerOff className="w-4 h-4 text-slate-500" />}
                            <span className={`font-mono text-sm font-bold ${mod.isActive ? 'text-white' : 'text-slate-400'}`}>{mod.name}</span>
                          </div>
                          <span className="text-[10px] uppercase font-bold tracking-widest px-2 py-0.5 rounded bg-white/5 text-slate-400 border border-white/5">
                            {mod.category}
                          </span>
                        </div>
                        
                        <div className="flex items-center justify-between text-xs font-mono">
                          <span className="text-slate-500">Status</span>
                          {mod.isActive ? (
                            <span className="text-emerald-400 font-bold flex items-center gap-1.5"><Zap className="w-3 h-3" /> ACTIVE</span>
                          ) : (
                            <span className="text-slate-600">IDLE (UNLOADED)</span>
                          )}
                        </div>
                        
                        {mod.isActive && (
                          <div className="w-full bg-black rounded-full h-1 mt-1 border border-white/5">
                            <motion.div 
                              className="bg-atc-purple h-full rounded-full" 
                              initial={{ width: 0 }}
                              animate={{ width: `${mod.loadAvg}%` }}
                              transition={{ type: 'spring', stiffness: 50, damping: 20 }}
                            />
                          </div>
                        )}
                      </motion.div>
                    ))}
                  </AnimatePresence>
                </div>
              </div>
            </div>

            <div className="col-span-1 lg:col-span-4 flex flex-col gap-6">
              <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col gap-4 relative overflow-hidden">
                 <div className="absolute top-0 right-0 w-32 h-32 bg-atc-cyan/5 rounded-full blur-[50px] pointer-events-none" />
                 <h3 className="text-sm font-bold font-mono tracking-widest text-slate-400 uppercase mb-2">Core Kernel Load</h3>
                 <div className="flex items-end gap-2 text-white">
                    <span className="text-5xl font-light tracking-tighter">
                       {Math.max(0, modules.filter(m => m.isActive).length * 10 - 5)}<span className="text-2xl text-slate-500">%</span>
                    </span>
                 </div>
                 <p className="text-xs text-slate-500">CPU Usage by active Sandbox Layers</p>
              </div>

              <div className="p-6 rounded-2xl bg-indigo-500/10 border border-indigo-500/20 text-indigo-200 text-sm">
                 <h3 className="font-bold flex items-center gap-2 mb-2"><Cpu className="w-4 h-4" /> Architektur-Prinzip</h3>
                 <p className="opacity-80 font-light leading-relaxed mb-4">
                   Das System besteht nicht aus einem monolithischen Kern. Stattdessen existiert ein minimaler <b>Orchestrator</b>, der zur Laufzeit benötigte Prozesse (z.B. P2P Gossip, Zero Knowledge Zirkuit Prüfer) als isolierte Container in den virtuellen Addressraum lädt.
                 </p>
                 <p className="opacity-80 font-light leading-relaxed">
                   Dies garantiert Null-Ressourcenverbrauch wenn ein Layer nicht gebraucht wird, bei maximaler Isolation der Kern-Routinen.
                 </p>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'compiler' && (
          <div className="flex flex-col gap-8">
            <div className="w-full relative py-8 px-4 flex flex-col gap-12 bg-[#090b14] border border-white/5 rounded-2xl">
              {/* Vertical line connecting nodes */}
              <div className="absolute left-8 lg:left-1/2 top-16 bottom-16 w-px bg-gradient-to-b from-atc-cyan via-emerald-400 to-atc-purple -translate-x-1/2 opacity-30 hidden lg:block" />
              
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-16 relative">
                 <div className="lg:text-right flex flex-col justify-center">
                    <h3 className="text-xl font-bold text-atc-cyan mb-2">1. Lexical Analysis (ATC-Lexer)</h3>
                    <p className="text-slate-400 text-sm leading-relaxed max-w-md ml-auto">
                      Der Rohtext wird vollständig durch den eigenen Regex-free State Machine Tokenizer analysiert. Das System wandelt Strings in hoch-effiziente enumerierte `ATCToken` Objekte um, die bereits erste Meta-Informationen für die Fehlerbehandlung tragen.
                    </p>
                 </div>
                 <div className="bg-black/60 p-4 border border-white/10 rounded-xl font-mono text-xs overflow-x-auto text-slate-300">
                    <pre><span className="text-fuchsia-400">{"["}</span>
  {"{ type: 'KEYWORD', value: 'layer', line: 1 }"},
  {"{ type: 'IDENTIFIER', value: 'Token', line: 1 }"},
  {"{ type: 'LBRACE', value: '{', line: 1 }"},
  {"{ type: 'KEYWORD', value: 'field', line: 2 }"},
  {"{ type: 'IDENTIFIER', value: 'totalSupply', line: 2 }"},
  {"{ type: 'COLON', value: ':', line: 2 }"},
  {"{ type: 'TYPE', value: 'u256', line: 2 }"},
  <span className="text-fuchsia-400">{"]"}</span></pre>
                 </div>
              </div>

              <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-16 relative">
                 <div className="bg-black/60 p-4 border border-white/10 rounded-xl font-mono text-xs overflow-x-auto text-slate-300 lg:order-1 order-2">
                    <pre><span className="text-emerald-400">{"{"}</span>
  <span className="text-indigo-300">"type"</span>: <span className="text-amber-300">"LayerDecl"</span>,
  <span className="text-indigo-300">"id"</span>: <span className="text-amber-300">"Token"</span>,
  <span className="text-indigo-300">"body"</span>: <span className="text-fuchsia-400">{"["}</span>
    <span className="text-emerald-400">{"{"}</span>
      <span className="text-indigo-300">"type"</span>: <span className="text-amber-300">"FieldDecl"</span>,
      <span className="text-indigo-300">"id"</span>: <span className="text-amber-300">"totalSupply"</span>,
      <span className="text-indigo-300">"valueType"</span>: <span className="text-amber-300">"u256"</span>
    <span className="text-emerald-400">{"}"}</span>
  <span className="text-fuchsia-400">{"]"}</span>
<span className="text-emerald-400">{"}"}</span></pre>
                 </div>
                 <div className="flex flex-col justify-center lg:order-2 order-1">
                    <h3 className="text-xl font-bold text-emerald-400 mb-2">2. Parser & AST Generation</h3>
                    <p className="text-slate-400 text-sm leading-relaxed max-w-md">
                      Mithilfe eines rekurrenten Abstieg-Parsers (Recursive Descent Parser) wird der Token-Stream in den Abstract Syntax Tree (AST) aufgeschlüsselt. Dies ist das Herzstück des Compilers, wo Syntax-Regeln geprüft und Semantik-Fehler frühzeitig mit präzisen Fehlermeldungen abgefangen werden.
                    </p>
                 </div>
              </div>

              <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-16 relative">
                 <div className="lg:text-right flex flex-col justify-center">
                    <h3 className="text-xl font-bold text-atc-purple mb-2">3. Bytecode Emission (ATVM)</h3>
                    <p className="text-slate-400 text-sm leading-relaxed max-w-md ml-auto">
                      Der validierte AST wird flachgeklopft und direkt in den minimalen Operationscode der ATC Virtual Machine iteriert. Hier finden Optimierungen wie Dead Code Elimination und Loop-Unrolling statt. Das Ergebnis ist reiner Opcode in HEX für die Blockausführung.
                    </p>
                 </div>
                 <div className="bg-black/60 p-4 border border-white/10 rounded-xl font-mono text-xs overflow-x-auto text-slate-300">
                    <pre><span className="text-slate-500">// ATVM Native Opcode Map</span>
<span className="text-amber-200">0x00</span> <span className="text-slate-400">OP_LAYER_INIT "Token"</span>
<span className="text-amber-200">0x12</span> <span className="text-slate-400">OP_ALLOC_MEM 32</span>
<span className="text-amber-200">0x0A</span> <span className="text-slate-400">OP_STORE_FIELD 0x01</span>
<span className="text-amber-200">0xFF</span> <span className="text-slate-400">OP_RETURN</span>
<span className="text-indigo-400">// Final Hex Output:</span>
00 1D 54 6F 6B 65 6E 12 20 0A 01 FF</pre>
                 </div>
              </div>

            </div>
          </div>
        )}

        {activeTab === 'syntax' && (
          <div className="flex flex-col gap-6">
             <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="bg-[#090b14] border border-white/5 rounded-xl overflow-hidden shadow-lg p-6 flex flex-col gap-4">
                   <h3 className="text-lg font-bold text-emerald-400 border-b border-white/10 pb-3 flex items-center gap-2">
                     <FileText className="w-5 h-5"/> Eigene Language Primitives
                   </h3>
                   <div className="flex flex-col gap-3">
                      <div className="bg-black/40 border border-white/5 p-3 rounded text-sm">
                         <span className="font-mono text-emerald-300 font-bold">layer</span> <span className="text-slate-300">Name {'{}'}</span>
                         <p className="text-xs text-slate-500 mt-1">Ersetzt "contract" oder "class". Stellt einen modularen Smart Contract dar.</p>
                      </div>
                      <div className="bg-black/40 border border-white/5 p-3 rounded text-sm">
                         <span className="font-mono text-emerald-300 font-bold">routine</span> <span className="text-slate-300">funcName() {'->'} type</span>
                         <p className="text-xs text-slate-500 mt-1">Ersetzt "function". Steht für deterministische, berechenbare Ausführungseinheiten auf der VM.</p>
                      </div>
                      <div className="bg-black/40 border border-white/5 p-3 rounded text-sm">
                         <span className="font-mono text-amber-300 font-bold">@zk_proof</span>
                         <p className="text-xs text-slate-500 mt-1">Nativer Kompilier-Dekorator für Routinen, die einen Zero Knowledge Proof ansteuern statt State auszuführen.</p>
                      </div>
                   </div>
                </div>

                <div className="bg-[#090b14] border border-white/5 rounded-xl overflow-hidden shadow-lg p-6 flex flex-col gap-4">
                   <h3 className="text-lg font-bold text-atc-cyan border-b border-white/10 pb-3 flex items-center gap-2">
                     <Code2 className="w-5 h-5"/> Einzigartige Typ-Sicherheit
                   </h3>
                   <p className="text-sm text-slate-400 leading-relaxed mb-2">
                      ATC-Lang unterstützt nicht nur primitive Datentypen, sondern implementiert auf Compiler-Ebene native Ressourcen-Typen, um Reentrancy und Double-Spending physisch unmöglich zu machen. Assets bewegen sich als physikalische <span className="font-mono text-atc-cyan">§Resource</span> Typen und können nicht kopiert (<span className="font-mono text-xs">clone</span>), sondern nur bewegt werden (<span className="font-mono text-xs">move</span>).
                   </p>
                   <pre className="p-3 bg-black/60 rounded border border-white/5 font-mono text-[11px] text-slate-300 leading-relaxed">
<span className="text-emerald-400">routine</span> <span className="text-indigo-300">send_tokens</span>(<span className="text-amber-200">amount</span>: <span className="text-rose-300">§Asset</span>) {'->'} bool {'{'}
  <span className="text-slate-500">// Compiler error: "Asset cannot be cloned"
  // let x = amount;</span>
  
  <span className="text-slate-500">// Nur 'move' in neuen State erlaubt:</span>
  Receiver.balances.push(<span className="text-emerald-400">move</span> amount);
  <span className="text-emerald-400">return</span> true;
{'}'}</pre>
                   {/* INTERAKTIVE LANGUAGE SPECIFICATION CHEAT SHEET */}
                </div>
             </div>
             {/* INTERAKTIVE LANGUAGE SPECIFICATION CHEAT SHEET */}
             <div className="bg-[#090b14] border border-white/5 rounded-2xl p-6 shadow-xl flex flex-col gap-6 mt-4">
                <div>
                   <h3 className="text-lg font-bold text-white flex items-center gap-2">
                      <Code2 className="w-5 h-5 text-indigo-400" />
                      atc-lang Syntax-Spezifikation & Referenz
                   </h3>
                   <p className="text-xs text-slate-400 mt-1">
                      Nutze diese offizielle Sprachdefinition für die Erstellung dezentraler Smart-Contract Logik auf der A-Town Virtual Machine (ATVM).
                   </p>
                </div>

                {/* Sub-Tabs Selector */}
                <div className="flex flex-wrap gap-2 border-b border-white/5 pb-4">
                   {SPEC_CONCEPTS.map((concept) => (
                      <button
                         key={concept.id}
                         onClick={() => setSelectedSpec(concept.id)}
                         className={`px-3 py-1.5 rounded-xl text-xs font-mono font-bold transition-all ${
                            selectedSpec === concept.id
                               ? 'bg-gradient-to-r from-indigo-500/10 to-teal-500/10 text-teal-400 border border-teal-500/30'
                               : 'bg-black/20 text-slate-400 hover:text-slate-200 hover:bg-black/40 border border-transparent'
                         }`}
                      >
                         {concept.name.split(". ")[1]}
                      </button>
                   ))}
                </div>

                {/* Spec details representation */}
                {(() => {
                   const spec = SPEC_CONCEPTS.find(c => c.id === selectedSpec) || SPEC_CONCEPTS[0];
                   return (
                      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
                         {/* Description and Rules */}
                         <div className="lg:col-span-5 flex flex-col gap-4">
                            <h4 className="text-sm font-bold text-teal-400 font-mono tracking-wider uppercase">
                               {spec.name}
                            </h4>
                            <p className="text-sm text-slate-300 leading-relaxed font-light">
                               {spec.desc}
                            </p>
                            
                            <div className="p-4 rounded-xl bg-black/40 border border-white/5 text-xs text-slate-400 flex flex-col gap-2.5">
                               <div className="font-bold text-slate-300 uppercase tracking-wider text-[10px] border-b border-white/5 pb-1.5">
                                  Compiler-Regeln
                                </div>
                               {spec.id === 'vars' && (
                                  <ul className="list-disc list-inside space-y-1">
                                     <li>Variablen sind standardmäßig <span className="text-amber-400 font-mono">immutable</span></li>
                                     <li>Dynamische Neuzuweisung erfordert das <span className="text-teal-400 font-mono">mut</span> Keyword</li>
                                     <li>Lineare Typen stellen sicher, dass Ressourcen nicht kopiert werden</li>
                                  </ul>
                               )}
                               {spec.id === 'funcs' && (
                                  <ul className="list-disc list-inside space-y-1">
                                     <li>Routinen nutzen das <span className="text-fuchsia-400 font-mono">routine</span> Keyword (Ersatz für function)</li>
                                     <li>Rückgabetypen folgen nach dem Pfeil-Symbol: <span className="text-teal-400 font-mono font-bold">{"->"} [Typ]</span></li>
                                     <li>Kryptografische Signierung via <span className="text-amber-400 font-mono">@zk_proof</span> ist nativ integrierbar</li>
                                  </ul>
                               )}
                               {spec.id === 'control' && (
                                  <ul className="list-disc list-inside space-y-1">
                                     <li>Conditionals (<span className="text-indigo-400 font-mono">if/else</span>) verlangen keine runden Klammern</li>
                                     <li>Bereichs-Loops sind deterministisch und gas-limitiert</li>
                                     <li>Keine unendlichen Open-End-Schleifen im Consens erlaubt</li>
                                  </ul>
                               )}
                               {spec.id === 'data' && (
                                  <ul className="list-disc list-inside space-y-1">
                                     <li>Standard-Listen verwenden generischen Typen: <span className="text-emerald-400 font-mono">List{"<T>"}</span></li>
                                     <li>Key-Value Speicher nutzen <span className="text-emerald-400 font-mono">Map{"<K,V>"}</span></li>
                                     <li>Nativer Null-Coalescing-Operator <span className="text-teal-400 font-mono font-bold">?:</span> fängt ungesetzte Werte ab</li>
                                  </ul>
                               )}
                               {spec.id === 'bitwise' && (
                                  <ul className="list-disc list-inside space-y-1">
                                     <li>Klassische Bitwise-Operatoren: <span className="text-amber-400 font-mono font-bold">& | ^ {"<<"} {">>"}</span></li>
                                     <li>Integrierter Überlaufschutz verhindert logische Blockfehler</li>
                                     <li>Optimierte <span className="text-teal-400 font-mono text-xs">rotate_left/right</span> Compiler Macros für SHA-256</li>
                                  </ul>
                               )}
                               {spec.id === 'structs' && (
                                  <ul className="list-disc list-inside space-y-1">
                                     <li>Objekstrukturen werden als <span className="text-indigo-400 font-mono">struct</span> deklariert</li>
                                     <li>Initialisierung erfolgt explizit über <span className="text-teal-400 font-mono font-bold">init [StructName] {"{...}"}</span></li>
                                     <li>Datenfelder sind ohne Setter-Funktionen schreibgeschützt</li>
                                  </ul>
                               )}
                            </div>
                         </div>

                         {/* Sandbox Code Preview */}
                         <div className="lg:col-span-7 flex flex-col border border-white/10 rounded-xl overflow-hidden shadow-2xl bg-[#04060b]">
                            {/* Browser Bar */}
                            <div className="flex items-center justify-between px-4 h-9 bg-black/40 border-b border-white/5 text-slate-500 text-[10px] font-mono">
                               <div className="flex items-center gap-1.5">
                                  <div className="w-2.5 h-2.5 rounded-full bg-red-500/40" />
                                  <div className="w-2.5 h-2.5 rounded-full bg-yellow-500/40" />
                                  <div className="w-2.5 h-2.5 rounded-full bg-green-500/40" />
                                </div>
                               <span className="text-slate-400 tracking-wider">compiler_spec_{spec.id}.atc</span>
                               <span className="text-[10px] bg-indigo-500/10 text-indigo-400 font-bold px-1.5 py-0.5 rounded uppercase tracking-widest">
                                  atc-lang
                               </span>
                            </div>
                            {/* Code lines */}
                            <div className="p-4 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto text-[11px] select-all scrollbar-thin max-h-[320px]">
                               <pre className="whitespace-pre-wrap">{spec.code}</pre>
                            </div>
                         </div>
                      </div>
                   );
                })()}

             </div>

             <div className="mt-4 p-6 bg-emerald-500/10 border border-emerald-500/20 rounded-xl relative overflow-hidden">
                <div className="absolute right-0 top-0 bottom-0 w-1/3 bg-gradient-to-l from-emerald-500/10 to-transparent pointer-events-none" />
                <h3 className="text-xl font-bold text-emerald-400 mb-2">Die ATC-Lexer Spezifikation (EBNF Grammar Subset)</h3>
                <p className="text-sm text-emerald-200/70 mb-4 font-light max-w-4xl">
                   Ein exklusiver Einblick in die Syntaxkonstruktion unserer selbstgeschriebenen Sprache. Jedes Keyword wird vom Parser in der AST-Phase validiert.
                </p>
                <pre className="font-mono text-xs text-emerald-300/80 leading-loose">
Program      ::= (Declaration)*
Declaration  ::= LayerDecl | InterfaceDecl
LayerDecl    ::= "layer" Identifier "{" (FieldDecl | RoutineDecl)* "}"
FieldDecl    ::= "field" Identifier ":" Type ";"
RoutineDecl  ::= ("@zk_proof")? ("public" | "private")? "routine" Identifier "(" Params? ")" ({"\"->\""} Type)? Block
Block        ::= "{" Statement* "}"
Statement    ::= LetStmt | Assignment | EmitStmt | ReturnStmt | ExpressionStmt
Type         ::= "u64" | "u256" | "bool" | "Address" | "§Asset" | MapType
              </pre>
             </div>
          </div>
        )}
      </div>
    </div>
  );
}

