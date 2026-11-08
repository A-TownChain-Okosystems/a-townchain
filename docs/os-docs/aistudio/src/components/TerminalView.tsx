import React, { useState, useRef, useEffect } from 'react';
import { Terminal, Mic, Hash } from 'lucide-react';

export function TerminalView() {
  const [activeChannel, setActiveChannel] = useState<'main' | 'network' | 'security'>('main');
  const [history, setHistory] = useState<Record<string, string[]>>({
    main: [
      "ATC-OS Terminal v1.0.0 [Main Channel]",
      "Loading kernel modules...",
      "[OK] Boot Sequence completed.",
      "Type 'help' for a list of commands."
    ],
    network: [
      "Network Sub-Channel initialized.",
      "Awaiting socket connections..."
    ],
    security: [
      "Security & Audit Sub-Channel.",
      "Zero-Knowledge Snark proof listener connected."
    ]
  });

  const [input, setInput] = useState('');
  const [commandHistoryList, setCommandHistoryList] = useState<string[]>(() => {
    try {
      return JSON.parse(localStorage.getItem('atc_terminal_commands') || '[]');
    } catch {
      return [];
    }
  });
  const [historyIndex, setHistoryIndex] = useState(-1);
  const inputRef = useRef<HTMLInputElement>(null);

  // Auto scroll
  const bottomRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [history, activeChannel]);

  const processCommand = (cmdStr: string) => {
    const cmd = cmdStr.trim();
    if (!cmd) return;

    const newList = [cmd, ...commandHistoryList].slice(0, 50);
    setCommandHistoryList(newList);
    localStorage.setItem('atc_terminal_commands', JSON.stringify(newList));
    setHistoryIndex(-1);

    const newHistoryData = { ...history };
    const currentChannelHistory = [...newHistoryData[activeChannel], `user@atc-os-[${activeChannel}]:~$ ${cmd}`];

    switch (cmd.toLowerCase().replace(/[^a-z0-9-]/g, '')) {
      case 'help':
        currentChannelHistory.push("Available commands: help, clear, status, ping, nodeinfo");
        break;
      case 'clear':
        newHistoryData[activeChannel] = [];
        setHistory(newHistoryData);
        setInput('');
        return;
      case 'status':
        currentChannelHistory.push("System Status: All services online. ATC Node synced.");
        break;
      case 'ping':
        currentChannelHistory.push("PONG [12ms]");
        break;
      case 'nodeinfo':
        currentChannelHistory.push("Node ID: atc_node_9x2b... / Active Peers: 42");
        break;
      default:
        currentChannelHistory.push(`Command not found: ${cmd}`);
    }

    newHistoryData[activeChannel] = currentChannelHistory;
    setHistory(newHistoryData);
  };

  const handleCommand = (e: React.FormEvent) => {
    e.preventDefault();
    processCommand(input);
    setInput('');
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (historyIndex < commandHistoryList.length - 1) {
        const nextIndex = historyIndex + 1;
        setHistoryIndex(nextIndex);
        setInput(commandHistoryList[nextIndex]);
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (historyIndex > 0) {
        const prevIndex = historyIndex - 1;
        setHistoryIndex(prevIndex);
        setInput(commandHistoryList[prevIndex]);
      } else if (historyIndex === 0) {
        setHistoryIndex(-1);
        setInput('');
      }
    }
  };

  const handleVoiceCommand = () => {
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    if (!SpeechRecognition) {
      setHistory(prev => ({
        ...prev,
        [activeChannel]: [...prev[activeChannel], "Speech Recognition is not supported dynamically in this browser."]
      }));
      return;
    }
    const recognition = new SpeechRecognition();
    recognition.lang = 'en-US';
    recognition.start();

    // Flash placeholder text
    const prevInput = input;
    setInput('[Listening... Speak command]');

    recognition.onresult = (event: any) => {
      const transcript = event.results[0][0].transcript;
      setInput('');
      processCommand(transcript);
    };

    recognition.onerror = () => {
      setInput(prevInput);
      setHistory(prev => ({
        ...prev,
        [activeChannel]: [...prev[activeChannel], "[Error: Voice recognition failed or aborted]"]
      }));
    };
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-emerald-400 font-mono text-sm overflow-hidden rounded-xl border border-white/10 relative">
      <div className="flex items-center justify-between p-4 bg-black/40 border-b border-white/5">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 text-emerald-500/50">
            <Terminal className="w-5 h-5" />
            <span className="hidden sm:inline">ATC-Shell</span>
          </div>
          <div className="flex bg-black/40 p-1 rounded-lg border border-white/5">
            {(['main', 'network', 'security'] as const).map(ch => (
              <button
                key={ch}
                onClick={() => setActiveChannel(ch)}
                className={`flex items-center gap-1.5 px-3 py-1 text-xs rounded transition-colors uppercase tracking-wider ${
                  activeChannel === ch 
                    ? 'bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/30' 
                    : 'text-slate-500 hover:text-emerald-400'
                }`}
              >
                <Hash className="w-3 h-3" />
                {ch}
              </button>
            ))}
          </div>
        </div>
        <button onClick={handleVoiceCommand} className="p-1 hover:bg-white/10 rounded transition-colors text-emerald-500/80 hover:text-emerald-400" title="Voice Command">
          <Mic className="w-4 h-4" />
        </button>
      </div>
      
      <div className="flex-1 overflow-y-auto p-4 space-y-1 custom-scrollbar">
        {history[activeChannel].map((line, i) => (
          <div key={i} className="break-all">{line}</div>
        ))}
        <div ref={bottomRef} />
      </div>

      <form onSubmit={handleCommand} className="flex gap-2 items-center p-4 bg-black/40 border-t border-white/5">
        <span className="text-emerald-500 hidden sm:inline">user@atc-os-[{activeChannel}]:~$</span>
        <span className="text-emerald-500 sm:hidden">~$</span>
        <input 
          ref={inputRef}
          autoFocus
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          className="flex-1 bg-transparent outline-none border-none text-emerald-400"
          spellCheck={false}
        />
      </form>
    </div>
  );
}
