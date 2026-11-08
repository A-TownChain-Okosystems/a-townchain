import React, { useState, useRef, useEffect } from "react";
import { useUISounds } from "../App";
import { Search } from "lucide-react";

export function TerminalPanel() {
  const playUISound = useUISounds();
  const [history, setHistory] = useState([
    { id: "t1", text: "ATOS Terminal v1.2.0 initialized.", isCmd: false },
    { id: "t2", text: 'Type "help" for a list of commands.', isCmd: false },
  ]);
  const [input, setInput] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [history]);

  const handleCommand = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      playUISound('click');
      const cmd = input.trim();
      setInput("");
      if (!cmd) return;

      const newHistory = [
        ...history,
        { id: Date.now().toString(), text: `$ ${cmd}`, isCmd: true },
      ];

      let response = "";
      switch (cmd.toLowerCase()) {
        case "help":
          response =
            "Available commands:\\nhelp - Show this message\\nclear - Clear terminal\\nnode - Run Node.js\\nlumino - Run Lumino compiler\\nls - List resources\\nping - Ping network";
          break;
        case "clear":
          setHistory([]);
          return;
        case "ls":
          response = "src/   docs/   config/   public/";
          break;
        case "ping":
          response =
            "Pinging ATOS backbone... \\nReply from 192.168.0.1: bytes=32 time=4ms TTL=54";
          break;
        case "node -v":
        case "node --version":
          response = "v20.10.0";
          break;
        case "lumino":
          response = "Lumino compiler usage: lumino <file.lm> [--target out]";
          break;
        default:
          response = `Command not found: ${cmd}`;
      }

      const lines = response.split("\\n");
      lines.forEach((line, i) => {
        newHistory.push({
          id: `${Date.now()}_res_${i}`,
          text: line,
          isCmd: false,
        });
      });

      setHistory(newHistory);
    }
  };

  const filteredHistory = history.filter(item => 
    item.text.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex flex-col h-full font-mono text-xs overflow-hidden">
      <div className="flex items-center gap-2 mb-2 p-1.5 bg-black/40 border border-white/5 rounded-md shrink-0">
        <Search className="w-3 h-3 text-slate-500" />
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="Filter output..."
          className="flex-1 bg-transparent border-none outline-none text-slate-300 placeholder-slate-600 font-sans text-xs"
        />
      </div>
      <div className="flex-1 overflow-y-auto">
        {filteredHistory.map((item) => (
          <div
            key={item.id}
            className={`${item.isCmd ? "text-slate-300" : "text-slate-400"}`}
          >
            {item.text}
          </div>
        ))}
        <div className="flex items-center gap-2 text-emerald-400 mt-1">
          <span>$</span>
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleCommand}
            className="flex-1 bg-transparent border-none outline-none text-slate-200"
            autoFocus
            spellCheck={false}
            autoComplete="off"
          />
        </div>
        <div ref={endRef} />
      </div>
    </div>
  );
}
